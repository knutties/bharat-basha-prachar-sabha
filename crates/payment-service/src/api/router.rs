use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::AppState;

use super::handlers;

/// Build the Axum router with all payment service routes.
pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/v1/plans", get(handlers::list_plans))
        .route(
            "/api/v1/subscriptions",
            post(handlers::create_subscription),
        )
        .route(
            "/api/v1/subscriptions/{id}",
            get(handlers::get_subscription),
        )
        .route(
            "/api/v1/subscriptions/{id}/cancel",
            post(handlers::cancel_subscription),
        )
        .route(
            "/api/v1/webhooks/razorpay",
            post(handlers::razorpay_webhook),
        )
        .route("/health", get(handlers::health_check))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
