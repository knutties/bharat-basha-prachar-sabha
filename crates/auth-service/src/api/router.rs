use axum::{
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::AppState;

use super::handlers;

/// Build the Axum router with all auth service routes.
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/v1/auth/register/student",
            post(handlers::register_student),
        )
        .route(
            "/api/v1/auth/register/teacher",
            post(handlers::register_teacher),
        )
        .route("/api/v1/auth/login", post(handlers::login))
        .route("/api/v1/auth/refresh", post(handlers::refresh_token))
        .route(
            "/api/v1/auth/profile/{user_id}",
            get(handlers::get_profile).put(handlers::update_profile),
        )
        .route("/health", get(handlers::health_check))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
