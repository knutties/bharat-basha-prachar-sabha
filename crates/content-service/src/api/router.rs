use axum::{
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::AppState;

use super::handlers;

/// Build the Axum router with all content service routes.
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/v1/content",
            post(handlers::create_content).get(handlers::list_content),
        )
        .route(
            "/api/v1/content/{id}",
            get(handlers::get_content).put(handlers::update_content),
        )
        .route(
            "/api/v1/content/{id}/review",
            post(handlers::create_review),
        )
        .route("/api/v1/media/upload", post(handlers::upload_media))
        .route("/health", get(handlers::health_check))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
