use std::net::SocketAddr;
use std::sync::Arc;
use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use tokio::sync::broadcast::{self, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use application::services::{DependOnCreateEmptyInstanceService, DependOnCreateRingService};
use kernel::external::uuid::Uuid;
use kernel::repository::{DependOnInstanceRepository, InstanceRepository};
use crate::AppHandler;
use crate::controller::{Controller, CreateRingRequest, InstanceToDetailResponse, MaybeInstanceToDetailResponse, RequestToCreateRingDto, RingDtoToDetailResponseJson, SelectionIdToLocationId};

static BROADCAST: Lazy<Sender<String>> = Lazy::new(|| broadcast::channel(10).0);

#[allow(unused_mut)]
pub async fn handle(mut socket: WebSocket, who: SocketAddr, handler: AppHandler, location: Uuid) {
    let (mut sen, mut rec) = socket.split();
    let arc_sen = Arc::new(Mutex::new(sen));

    let handler_once = handler.clone();
    let instance = match Controller::new(SelectionIdToLocationId, MaybeInstanceToDetailResponse)
        .intake(location)
        .handle(|input| async move { handler_once.as_ref().instance_repository().find_unfinished(&input).await })
        .await
    {
        Ok(Some(res)) => res,
        _ => {
            let handler_once = handler.clone();
            tracing::info!("`{who}` request location_id: {:?} but there were no valid instances.", location);
            let Ok(instance) = Controller::new(SelectionIdToLocationId, InstanceToDetailResponse)
                .intake(location)
                .handle(|input| async move {
                    use application::services::CreateEmptyInstanceService;
                    handler_once.as_ref().create_empty_instance_service().create(input).await
                })
                .await
            else {
                arc_sen.lock().await.send(Message::Text(format!("Failed generate instance in `{location}`."))).await.unwrap();
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
    let mut btx = BROADCAST.clone();

    let mut tx1 = Arc::clone(&arc_sen);
    let handler_recv = handler.clone();
    let mut recv_task: JoinHandle<()> = tokio::spawn(async move {
        while let Some(Ok(Message::Text(msg))) = rec.next().await {
            tracing::debug!("`{who}` sent: {:?}", msg);
            let Ok(deserialized) = serde_json::from_str::<CreateRingRequest>(&msg) else {
                tracing::error!("`{who}` sent invalid JSON: {:?}", msg);
                let _ = tx1.lock().await.send(Message::Text("Invalid JSON".to_string())).await;
                continue;
            };

            let res = match Controller::new(RequestToCreateRingDto, RingDtoToDetailResponseJson)
                .intake(deserialized)
                .handle(|input| async {
                    use application::services::CreateRingService;
                    handler_recv.as_ref().create_ring_service().create(input).await
                })
                .await
            {
                Ok(res) => res,
                Err(e) => {
                    tracing::error!("`{who}` sent conflict data: {:?}", e);
                    let _ = tx1.lock().await.send(Message::Text(format!("Conflict data creation. reason: {e}"))).await;
                    continue;
                }
            };

            let Ok(serialized) = serde_json::to_string(&res) else {
                continue;
            };

            let _ = btx.send(serialized);
        }
        tracing::debug!("`{who}` lost connection.",);
    });


    let mut send_task: JoinHandle<()> = tokio::spawn(async move {
        while let Ok(msg) = brx.recv().await {
            if arc_sen.lock().await.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    tracing::debug!("`{}` disconnected", who);
}
