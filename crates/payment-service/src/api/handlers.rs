use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::PaymentError;
use crate::domain::models::CreateSubscriptionInput;
use crate::AppState;

// ─── Request / Response DTOs ────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateSubscriptionRequest {
    pub user_id: Uuid,
    pub plan_id: Uuid,
}

#[derive(Deserialize)]
pub struct RazorpayWebhookPayload {
    pub event: String,
    pub payload: serde_json::Value,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
}

// ─── Handlers ───────────────────────────────────────────────────

pub async fn list_plans(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match state.payment_service.list_plans() {
        Ok(plans) => (StatusCode::OK, Json(serde_json::json!(plans)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn create_subscription(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateSubscriptionRequest>,
) -> impl IntoResponse {
    let input = CreateSubscriptionInput {
        user_id: req.user_id,
        plan_id: req.plan_id,
    };

    match state.payment_service.create_subscription(input) {
        Ok(subscription) => {
            (StatusCode::CREATED, Json(serde_json::json!(subscription)))
                .into_response()
        }
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn get_subscription(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.payment_service.get_subscription(id) {
        Ok(subscription) => {
            (StatusCode::OK, Json(serde_json::json!(subscription)))
                .into_response()
        }
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn cancel_subscription(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.payment_service.cancel_subscription(id) {
        Ok(subscription) => {
            (StatusCode::OK, Json(serde_json::json!(subscription)))
                .into_response()
        }
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn razorpay_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RazorpayWebhookPayload>,
) -> impl IntoResponse {
    match state
        .payment_service
        .handle_razorpay_webhook(&payload.event, &payload.payload)
    {
        Ok(()) => (
            StatusCode::OK,
            Json(MessageResponse {
                message: "Webhook processed".to_string(),
            }),
        )
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "payment-service",
    }))
}

// ─── Error Mapping ──────────────────────────────────────────────

fn map_error(err: PaymentError) -> (StatusCode, Json<ErrorResponse>) {
    let (status, code) = match &err {
        PaymentError::PlanNotFound(_) => {
            (StatusCode::NOT_FOUND, "PLAN_NOT_FOUND")
        }
        PaymentError::SubscriptionNotFound(_) => {
            (StatusCode::NOT_FOUND, "SUBSCRIPTION_NOT_FOUND")
        }
        PaymentError::PaymentFailed(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "PAYMENT_FAILED")
        }
        PaymentError::AlreadySubscribed(_) => {
            (StatusCode::CONFLICT, "ALREADY_SUBSCRIBED")
        }
        PaymentError::WebhookValidationFailed(_) => {
            (StatusCode::BAD_REQUEST, "WEBHOOK_VALIDATION_FAILED")
        }
        PaymentError::ValidationError(_) => {
            (StatusCode::BAD_REQUEST, "VALIDATION_ERROR")
        }
        PaymentError::RepositoryError(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR")
        }
    };

    (
        status,
        Json(ErrorResponse {
            error: ErrorBody {
                code: code.to_string(),
                message: err.to_string(),
            },
        }),
    )
}
