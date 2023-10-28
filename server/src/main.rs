use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::{Router, Server};
use server::middleware::simple_auth;
use server::{routes, AppHandler};
use std::net::SocketAddr;
use tower_http::{cors::{CorsLayer, Any}, trace::TraceLayer};
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
                        "driver=debug,server=debug,tower_http=trace,hyper=trace,sqlx=debug".into()
                    }),
                ))
                .with_filter(tracing_subscriber::filter::LevelFilter::TRACE),
        )
        .with(
            tracing_subscriber::fmt::Layer::default()
                .with_writer(non_blocking_appender)
                .with_ansi(false)
                .with_filter(tracing_subscriber::filter::LevelFilter::DEBUG),
        )
        .init();

    let handler = AppHandler::init().await?;

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);


    let admin = Router::new()
        .route(
            "/",
            post(routes::reg_location)
                .patch(routes::upd_location)
                .delete(routes::del_location),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            handler.clone(),
            simple_auth,
        ));

    let image = Router::new()
        .route("/", post(routes::reg_images))
        .layer(DefaultBodyLimit::disable());

    let socket = Router::new()
        .route("/", get(routes::socket::ws_handler));

    let app = Router::new()
        .route("/locations", get(routes::locations))
        .nest("/locations", admin)
        .route("/rings", get(routes::rings).post(routes::reg_ring))
        .nest("/ws-rings", socket)
        .nest("/images", image)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(handler);

    let bind = SocketAddr::from(([0, 0, 0, 0], 3854));

    tracing::info!("WAWWD API server Starting...");
    tracing::info!("Server listening on {}", bind.to_string());

    Server::bind(&bind)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
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
