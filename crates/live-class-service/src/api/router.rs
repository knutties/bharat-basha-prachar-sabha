use axum::{
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::AppState;

use super::handlers;

/// Build the Axum router with all live class service routes.
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/v1/classes",
            post(handlers::create_class).get(handlers::list_classes),
        )
        .route("/api/v1/classes/{id}", get(handlers::get_class))
        .route(
            "/api/v1/classes/{id}/enroll",
            post(handlers::enroll_in_class),
        )
        .route(
            "/api/v1/teachers/{id}/availability",
            get(handlers::get_teacher_availability)
                .put(handlers::update_teacher_availability),
        )
        .route("/health", get(handlers::health_check))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
