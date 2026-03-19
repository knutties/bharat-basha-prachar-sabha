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

/// Build the Axum router with all auth service routes.
pub fn create_router(state: Arc<AppState>) -> Router {
    // Public routes — no authentication required
    let public_routes = Router::new()
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
        .route("/health", get(handlers::health_check));

    // Protected routes — require valid JWT
    let protected_routes = Router::new()
        .route(
            "/api/v1/auth/profile/:user_id",
            get(handlers::get_profile).put(handlers::update_profile),
        )
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ));

    public_routes
        .merge(protected_routes)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
