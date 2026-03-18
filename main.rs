mod models;
mod routes;

use axum::{Router, middleware};
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    compression::CompressionLayer,
    services::ServeDir,
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // ── Tracing / logging setup ──────────────────────────────────────────────
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ironweb=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ── CORS ─────────────────────────────────────────────────────────────────
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // ── Router ───────────────────────────────────────────────────────────────
    let app = Router::new()
        // API routes
        .nest("/api", routes::api::router())
        // Static files (./static/)
        .nest_service("/static", ServeDir::new("static"))
        // Serve index.html at "/"
        .fallback_service(ServeDir::new("static").append_index_html_on_directories(true))
        // Middleware stack
        .layer(cors)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    // ── Bind & serve ─────────────────────────────────────────────────────────
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("⚡ IronWeb listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
