use axum::{
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::AppState;

use super::handlers;

/// Build the Axum router with all notification service routes.
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/v1/notifications/send",
            post(handlers::send_notification),
        )
        .route(
            "/api/v1/notifications/{user_id}",
            get(handlers::get_notifications),
        )
        .route(
            "/api/v1/notifications/{id}/read",
            put(handlers::mark_as_read),
        )
        .route(
            "/api/v1/notifications/{user_id}/preferences",
            get(handlers::get_preferences)
                .put(handlers::update_preferences),
        )
        .route("/health", get(handlers::health_check))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
