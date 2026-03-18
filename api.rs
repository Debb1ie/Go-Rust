use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use crate::models::{ApiResponse, ContactAck, ContactForm, Project, ServerInfo};
use tracing::{info, warn};

/// Build and return the /api sub-router
pub fn router() -> Router {
    Router::new()
        .route("/",          get(index))
        .route("/health",    get(health))
        .route("/projects",  get(list_projects))
        .route("/projects/:id", get(get_project))
        .route("/contact",   post(contact))
}

// ── GET /api/ ────────────────────────────────────────────────────────────────

async fn index() -> impl IntoResponse {
    info!("GET /api/");
    Json(ApiResponse::ok(ServerInfo {
        name:         "IronWeb",
        version:      env!("CARGO_PKG_VERSION"),
        language:     "Rust",
        framework:    "Axum 0.7",
        uptime_hint:  "use GET /api/health for uptime",
    }))
}

// ── GET /api/health ──────────────────────────────────────────────────────────

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({
        "status":  "ok",
        "service": "ironweb",
        "lang":    "Rust",
    }))
}

// ── GET /api/projects ────────────────────────────────────────────────────────

async fn list_projects() -> impl IntoResponse {
    info!("GET /api/projects");
    let projects = sample_projects();
    Json(ApiResponse::ok(projects))
}

// ── GET /api/projects/:id ────────────────────────────────────────────────────

async fn get_project(Path(id): Path<u32>) -> impl IntoResponse {
    info!("GET /api/projects/{}", id);
    match sample_projects().into_iter().find(|p| p.id == id) {
        Some(p) => (StatusCode::OK, Json(ApiResponse::ok(p))).into_response(),
        None => {
            warn!("Project {} not found", id);
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "success": false,
                    "error":   format!("project {} not found", id),
                })),
            )
                .into_response()
        }
    }
}

// ── POST /api/contact ────────────────────────────────────────────────────────

async fn contact(Json(form): Json<ContactForm>) -> impl IntoResponse {
    info!("POST /api/contact from {}", form.email);

    // Basic validation
    if form.name.trim().is_empty() || form.email.trim().is_empty() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({
                "success": false,
                "error":   "name and email are required",
            })),
        )
            .into_response();
    }

    if !form.email.contains('@') {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({
                "success": false,
                "error":   "invalid email address",
            })),
        )
            .into_response();
    }

    (
        StatusCode::CREATED,
        Json(ApiResponse::ok(ContactAck {
            received: true,
            reply_to: form.email.clone(),
        })),
    )
        .into_response()
}

// ── Seed data ────────────────────────────────────────────────────────────────

fn sample_projects() -> Vec<Project> {
    vec![
        Project {
            id:          1,
            title:       "IronWeb".into(),
            description: "A blazing-fast web server written in Rust with Axum.".into(),
            language:    "Rust".into(),
            stars:       420,
            url:         "https://github.com/example/ironweb".into(),
        },
        Project {
            id:          2,
            title:       "ZeroCache".into(),
            description: "Lock-free in-memory cache with nanosecond latency.".into(),
            language:    "Rust".into(),
            stars:       1_800,
            url:         "https://github.com/example/zerocache".into(),
        },
        Project {
            id:          3,
            title:       "WasmEdge UI".into(),
            description: "React-like component system compiled to WebAssembly via Leptos.".into(),
            language:    "Rust".into(),
            stars:       760,
            url:         "https://github.com/example/wasmedge-ui".into(),
        },
    ]
}
