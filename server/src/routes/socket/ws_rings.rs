mod internal;
mod query;

use axum::extract::{ConnectInfo, Query, State, WebSocketUpgrade};
use axum::headers::UserAgent;
use axum::response::Response;
use axum::TypedHeader;
use std::net::SocketAddr;
use crate::AppHandler;

pub async fn ws_handler(
    State(handler): State<AppHandler>,
    ws: WebSocketUpgrade,
    agent: Option<TypedHeader<UserAgent>>,
    ConnectInfo(info): ConnectInfo<SocketAddr>,
    Query(query): Query<query::RequireQuery>
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
        internal::handle(socket, info, handler, query.location).await;
    })
}