use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};
use tokio::{net::TcpListener, signal};
use tower_http::trace::TraceLayer;
use uploader::routes::{health, upload};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    tracing::info!("Starting Uploader");

    let app = Router::new()
        // Healths
        .route("/livez", get(health::livez))
        .route("/readyz", get(health::livez))
        // Upload
        .route("/upload", post(upload::upload)
            .route_layer(DefaultBodyLimit::disable()))
        // Logging
        .layer(
            TraceLayer::new_for_http()
                .on_request(|request: &axum::http::Request<_>, _span: &tracing::Span| {
                    tracing::info!("Received {} {}", request.method(), request.uri());
                })
                .on_response(|response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                    tracing::info!("Response: {}, Latency: {}", response.status(), latency.as_millis());
                })
        )
        // Global Server Config
        ;
    let listener = TcpListener::bind("0.0.0.0:7878").await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("failed to listen for shutdown");
    tracing::info!("Signal received, shutting down...");
}
