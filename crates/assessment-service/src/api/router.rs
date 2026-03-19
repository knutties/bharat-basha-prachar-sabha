use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::AppState;

use super::handlers;

/// Build the Axum router with all assessment service routes.
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/assessments", post(handlers::create_assessment))
        .route(
            "/api/v1/assessments/{id}/submit",
            post(handlers::submit_assessment),
        )
        .route(
            "/api/v1/assessments/{id}/result",
            get(handlers::get_assessment_result),
        )
        .route(
            "/api/v1/students/{id}/credits",
            get(handlers::get_student_credits),
        )
        .route(
            "/api/v1/credentials/verify/{code}",
            get(handlers::verify_credential),
        )
        .route("/health", get(handlers::health_check))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
