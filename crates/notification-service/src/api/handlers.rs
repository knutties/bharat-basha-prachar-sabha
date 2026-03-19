use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::NotificationError;
use crate::domain::models::{
    SendNotificationInput, UpdatePreferencesInput,
};
use crate::AppState;

// ─── Request / Response DTOs ────────────────────────────────────

#[derive(Deserialize)]
pub struct SendNotificationRequest {
    pub user_id: Uuid,
    pub notification_type: String,
    pub channel: String,
    pub title: String,
    pub body: String,
}

#[derive(Deserialize)]
pub struct UpdatePreferencesRequest {
    pub email_enabled: bool,
    pub sms_enabled: bool,
    pub push_enabled: bool,
    pub digest_frequency: Option<String>,
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

pub async fn send_notification(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SendNotificationRequest>,
) -> impl IntoResponse {
    let input = SendNotificationInput {
        user_id: req.user_id,
        notification_type: req.notification_type,
        channel: req.channel,
        title: req.title,
        body: req.body,
    };

    match state.notification_service.send_notification(input) {
        Ok(notification) => {
            (StatusCode::CREATED, Json(serde_json::json!(notification)))
                .into_response()
        }
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn get_notifications(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.notification_service.get_notifications(user_id) {
        Ok(notifications) => {
            (StatusCode::OK, Json(serde_json::json!(notifications)))
                .into_response()
        }
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn mark_as_read(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.notification_service.mark_as_read(id) {
        Ok(()) => (
            StatusCode::OK,
            Json(MessageResponse {
                message: "Notification marked as read".to_string(),
            }),
        )
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn get_preferences(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.notification_service.get_preferences(user_id) {
        Ok(prefs) => (StatusCode::OK, Json(serde_json::json!(prefs)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn update_preferences(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<UpdatePreferencesRequest>,
) -> impl IntoResponse {
    let input = UpdatePreferencesInput {
        user_id,
        email_enabled: req.email_enabled,
        sms_enabled: req.sms_enabled,
        push_enabled: req.push_enabled,
        digest_frequency: req.digest_frequency,
    };

    match state.notification_service.update_preferences(input) {
        Ok(prefs) => (StatusCode::OK, Json(serde_json::json!(prefs)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "notification-service",
    }))
}

// ─── Error Mapping ──────────────────────────────────────────────

fn map_error(
    err: NotificationError,
) -> (StatusCode, Json<ErrorResponse>) {
    let (status, code) = match &err {
        NotificationError::NotificationNotFound(_) => {
            (StatusCode::NOT_FOUND, "NOT_FOUND")
        }
        NotificationError::UserNotFound(_) => {
            (StatusCode::NOT_FOUND, "USER_NOT_FOUND")
        }
        NotificationError::InvalidChannel(_) => {
            (StatusCode::BAD_REQUEST, "INVALID_CHANNEL")
        }
        NotificationError::DeliveryFailed(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "DELIVERY_FAILED")
        }
        NotificationError::ValidationError(_) => {
            (StatusCode::BAD_REQUEST, "VALIDATION_ERROR")
        }
        NotificationError::RepositoryError(_) => {
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
