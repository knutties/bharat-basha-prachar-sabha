use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::AppState;

use super::handlers;
use super::middleware::auth_middleware;

/// Build the Axum router with all curriculum service routes.
pub fn create_router(state: Arc<AppState>) -> Router {
    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/api/v1/languages", get(handlers::list_languages))
        .route(
            "/api/v1/languages/{language_id}/curriculum",
            get(handlers::get_curriculum),
        )
        .route(
            "/api/v1/languages/{language_id}/modules",
            get(handlers::list_modules),
        )
        .route("/api/v1/modules/{module_id}", get(handlers::get_module))
        .route("/health", get(handlers::health_check));

    // Protected routes (auth required)
    let protected_routes = Router::new()
        .route("/api/v1/lessons/{lesson_id}", get(handlers::get_lesson))
        .route(
            "/api/v1/enrollments",
            post(handlers::enroll_in_language),
        )
        .route(
            "/api/v1/enrollments/{enrollment_id}/learning-path",
            get(handlers::get_learning_path),
        )
        .route("/api/v1/progress", post(handlers::record_progress))
        .layer(middleware::from_fn(auth_middleware));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
