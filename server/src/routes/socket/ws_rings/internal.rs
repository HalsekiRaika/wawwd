use std::borrow::Cow;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::extract::ws::{CloseFrame, Message, WebSocket};
use futures::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use tokio::sync::broadcast::{self, Sender};
use tokio::sync::Mutex;
use tracing::Instrument;
use application::services::{DependOnCreateEmptyInstanceService, DependOnCreateRingService};
use kernel::external::uuid::Uuid;
use kernel::repository::{DependOnInstanceRepository, InstanceRepository};
use crate::AppHandler;
use crate::controller::{Controller, CreateRingRequestWithNonce, InstanceToDetailResponse, MaybeInstanceToDetailResponse, RequestToCreateRingDto, RingDtoToDetailResponseJson};

static BROADCAST: Lazy<Sender<String>> = Lazy::new(|| broadcast::channel(10).0);

pub async fn handle(socket: WebSocket, who: SocketAddr, handler: AppHandler, ctx: Uuid) {
    let (sen, mut rec) = socket.split();
    let arc_sen = Arc::new(Mutex::new(sen));

    let handler_once = handler.clone();
    let instance = match Controller::new((), MaybeInstanceToDetailResponse)
        .bypass(|| async move { handler_once.as_ref().instance_repository().find_unfinished().await })
        .instrument(tracing::debug_span!("initialize", ctx = %ctx))
        .await
    {
        Ok(Some(res)) => res,
        _ => {
            let handler_once = handler.clone();
            tracing::info!("`{who}` request but there were no valid instances.");
            let Ok(instance) = Controller::new((), InstanceToDetailResponse)
                .bypass(|| async move {
                    use application::services::CreateEmptyInstanceService;
                    handler_once.as_ref().create_empty_instance_service().create().await
                })
                .await
            else {
                let e = serde_json::to_string(&serde_json::json!({
                    "error": "instance_generate",
                    "reason": "Failed generate instance.",
                    "context_id": &ctx
                })).unwrap();
                arc_sen.lock().await.send(Message::Text(e)).await.unwrap();
                return;
            };
            instance
        }
    };

    let Ok(serialized) = serde_json::to_string(&instance) else {
        return;
    };

    arc_sen.lock().await
        .send(Message::Text(serialized))
        .await
        .unwrap();

    let mut brx = BROADCAST.subscribe();
    let btx = BROADCAST.clone();

    let tx1 = Arc::clone(&arc_sen);
    let handler_recv = handler.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = rec.next().await {
            if let Message::Text(msg) = msg {
                tracing::debug!("`{who}` sent: {:?}", msg);
                let Ok(deserialized) = serde_json::from_str::<CreateRingRequestWithNonce>(&msg) else {
                    tracing::error!("`{who}` sent invalid JSON: {:?}", msg);
                    let _ = tx1.lock().await.send(Message::Text("Invalid JSON".to_string())).await;
                    continue;
                };

                let mut res = match Controller::new(RequestToCreateRingDto, RingDtoToDetailResponseJson)
                    .intake(deserialized.req)
                    .handle(|input| async {
                        use application::services::CreateRingService;
                        handler_recv.as_ref().create_ring_service().create(input).await
                    })
                    .await
                {
                    Ok(res) => res,
                    Err(e) => {
                        tracing::error!("`{who}` sent invalid data: {:?}", e);
                        let e = e.to_string();
                        let e = serde_json::to_string(&serde_json::json!({
                            "error": "invalid_data",
                            "reason": e,
                            "context_id": &ctx
                        })).unwrap();
                        let _ = tx1.lock().await.send(Message::Text(e)).await;
                        continue;
                    }
                };

                res.nonce = deserialized.nonce;

                let Ok(serialized) = serde_json::to_string(&res) else {
                    continue;
                };

                let _ = btx.send(serialized);
            } else if let Message::Close(frame) = msg {
                match frame {
                    Some(_) => {
                        let _ = tx1.lock().await.send(Message::Close(Some(CloseFrame {
                            code: axum::extract::ws::close_code::NORMAL,
                            reason: Cow::from("client request"),
                        }))).await;
                        break;
                    }
                    None => {
                        tracing::warn!("`{}` somehow sent close message without CloseFrame", who);
                        break;
                    }
                }
            }
        }
        tracing::debug!("`{who}` lost connection.",);
    }.instrument(tracing::debug_span!("recv_task", ctx = %ctx)));


    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = brx.recv().await {
            if arc_sen.lock().await.send(Message::Text(msg)).await.is_err() {
                tracing::error!("abort task failed to send message.");
                break;
            }
        }
    }.instrument(tracing::debug_span!("send_task", ctx = %ctx)));

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    tracing::debug!("`{}` disconnected", who);
}
