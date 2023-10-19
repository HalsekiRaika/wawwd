use axum::routing::get;
use axum::{Router, Server};
use server::{routes, Handler};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let appender = tracing_appender::rolling::daily(std::path::Path::new("./logs/"), "debug.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_filter(tracing_subscriber::EnvFilter::new(
                    std::env::var("RUST_LOG").unwrap_or_else(|_| {
                        "driver=debug,server=debug,tower_http=debug,hyper=debug,sqlx=debug".into()
                    }),
                ))
                .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG),
        )
        .with(
            tracing_subscriber::fmt::Layer::default()
                .with_writer(non_blocking_appender)
                .with_ansi(false)
                .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG),
        )
        .init();

    let handler = Handler::init().await?;

    let app = Router::new()
        .route(
            "/locations",
            get(routes::locations)
                .post(routes::reg_location)
                .patch(routes::upd_location)
                .delete(routes::del_location),
        )
        .route("/rings",
            get(routes::rings)
                .post(routes::reg_ring)
        )
        .layer(TraceLayer::new_for_http())
        .with_state(handler);

    let bind = SocketAddr::from(([0, 0, 0, 0], 3854));

    tracing::info!("WAWWD API server Starting...");
    tracing::info!("Server listening on {}", bind.to_string());

    Server::bind(&bind)
        .serve(app.into_make_service())
        .with_graceful_shutdown(exit())
        .await?;

    Ok(())
}

async fn exit() {
    let user_interrupt = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install keyboard interrupt.")
    };

    tokio::select! {
        _ = user_interrupt => {}
    }
}
