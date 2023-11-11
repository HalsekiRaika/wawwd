mod internal;

use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::headers::UserAgent;
use axum::response::Response;
use axum::TypedHeader;
use std::net::SocketAddr;
use kernel::external::uuid::Uuid;
use crate::AppHandler;

pub async fn ws_handler(
    State(handler): State<AppHandler>,
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
    }).on_upgrade(move |socket| async move {
        let ctx = Uuid::new_v4();
        tracing::info!("`{user_agent}` at {info} websocket upgrade successfully. (context_id: {ctx})");
        internal::handle(socket, info, handler, ctx).await;
    })
}