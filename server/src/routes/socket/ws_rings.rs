use axum::extract::ws::WebSocket;
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::headers::UserAgent;
use axum::response::Response;
use axum::TypedHeader;
use std::net::SocketAddr;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    agent: Option<TypedHeader<UserAgent>>,
    ConnectInfo(info): ConnectInfo<SocketAddr>,
) -> Response {
    let user_agent = if let Some(TypedHeader(user_agent)) = agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    tracing::debug!("`{user_agent}` at {info} connected.");

    ws.on_failed_upgrade(|e| {
        tracing::error!("Failed to upgrade websocket: {}", e);
    }).on_upgrade(move |socket| handle(socket, info))
}

async fn handle(mut socket: WebSocket, who: SocketAddr) {
    tracing::debug!("`{}` disconnected", who);
}