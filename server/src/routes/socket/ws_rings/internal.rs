use std::net::SocketAddr;
use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use tokio::sync::broadcast::{self, Sender};
use tokio::task::JoinHandle;
use crate::AppHandler;

static BROADCAST: Lazy<Sender<String>> = Lazy::new(|| broadcast::channel(10).0);

#[allow(unused_mut)]
pub async fn handle(mut socket: WebSocket, who: SocketAddr, _handler: AppHandler) {
    let (mut sen, mut res) = socket.split();

    let mut rxm = BROADCAST.subscribe();
    let mut txm = BROADCAST.clone();

    let mut recv_task: JoinHandle<()> = tokio::spawn(async move {
        while let Some(Ok(Message::Text(msg))) = res.next().await {
            tracing::debug!("`{}` sent: {:?}", who, msg);
            let _ = txm.send(msg);
        }
        tracing::debug!("`{who}` lost connection.",);
    });


    let mut send_task: JoinHandle<()> = tokio::spawn(async move {
        while let Ok(msg) = rxm.recv().await {
            if sen.send(Message::Text(msg)).await.is_err() {
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
