use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::LiveClassError;
use crate::domain::models::{
    CreateClassInput, EnrollInput, TeacherAvailabilityInput,
};
use crate::AppState;

// ─── Request / Response DTOs ────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateClassRequest {
    pub teacher_id: Uuid,
    pub language_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub scheduled_at: String,
    pub duration_minutes: i32,
    pub max_students: i32,
}

#[derive(Deserialize)]
pub struct EnrollRequest {
    pub student_id: Uuid,
}

#[derive(Deserialize)]
pub struct AvailabilityRequest {
    pub slots: Vec<AvailabilitySlot>,
}

#[derive(Deserialize)]
pub struct AvailabilitySlot {
    pub day_of_week: i32,
    pub start_time: String,
    pub end_time: String,
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

pub async fn create_class(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateClassRequest>,
) -> impl IntoResponse {
    let scheduled_at = match chrono::DateTime::parse_from_rfc3339(&req.scheduled_at) {
        Ok(dt) => dt.with_timezone(&chrono::Utc),
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: "VALIDATION_ERROR".to_string(),
                        message: "Invalid date format, expected RFC3339"
                            .to_string(),
                    },
                }),
            )
                .into_response();
        }
    };

    let input = CreateClassInput {
        teacher_id: req.teacher_id,
        language_id: req.language_id,
        title: req.title,
        description: req.description,
        scheduled_at,
        duration_minutes: req.duration_minutes,
        max_students: req.max_students,
    };

    match state.live_class_service.create_class(input) {
        Ok(class) => (StatusCode::CREATED, Json(serde_json::json!(class)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn list_classes(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match state.live_class_service.list_classes() {
        Ok(classes) => (StatusCode::OK, Json(serde_json::json!(classes)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn get_class(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.live_class_service.get_class(id) {
        Ok(class) => (StatusCode::OK, Json(serde_json::json!(class)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn enroll_in_class(
    State(state): State<Arc<AppState>>,
    Path(class_id): Path<Uuid>,
    Json(req): Json<EnrollRequest>,
) -> impl IntoResponse {
    let input = EnrollInput {
        class_id,
        student_id: req.student_id,
    };

    match state.live_class_service.enroll(input) {
        Ok(enrollment) => {
            (StatusCode::CREATED, Json(serde_json::json!(enrollment)))
                .into_response()
        }
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn get_teacher_availability(
    State(state): State<Arc<AppState>>,
    Path(teacher_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.live_class_service.get_availability(teacher_id) {
        Ok(slots) => (StatusCode::OK, Json(serde_json::json!(slots)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn update_teacher_availability(
    State(state): State<Arc<AppState>>,
    Path(teacher_id): Path<Uuid>,
    Json(req): Json<AvailabilityRequest>,
) -> impl IntoResponse {
    let slots: Vec<TeacherAvailabilityInput> = req
        .slots
        .into_iter()
        .map(|s| TeacherAvailabilityInput {
            teacher_id,
            day_of_week: s.day_of_week,
            start_time: s.start_time,
            end_time: s.end_time,
        })
        .collect();

    match state
        .live_class_service
        .update_availability(teacher_id, slots)
    {
        Ok(availability) => {
            (StatusCode::OK, Json(serde_json::json!(availability)))
                .into_response()
        }
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "live-class-service",
    }))
}

// ─── Error Mapping ──────────────────────────────────────────────

fn map_error(err: LiveClassError) -> (StatusCode, Json<ErrorResponse>) {
    let (status, code) = match &err {
        LiveClassError::ClassNotFound(_) => {
            (StatusCode::NOT_FOUND, "NOT_FOUND")
        }
        LiveClassError::ClassFull(_) => {
            (StatusCode::CONFLICT, "CLASS_FULL")
        }
        LiveClassError::AlreadyEnrolled(_) => {
            (StatusCode::CONFLICT, "ALREADY_ENROLLED")
        }
        LiveClassError::TeacherNotFound(_) => {
            (StatusCode::NOT_FOUND, "TEACHER_NOT_FOUND")
        }
        LiveClassError::ScheduleConflict(_) => {
            (StatusCode::CONFLICT, "SCHEDULE_CONFLICT")
        }
        LiveClassError::ValidationError(_) => {
            (StatusCode::BAD_REQUEST, "VALIDATION_ERROR")
        }
        LiveClassError::RepositoryError(_) => {
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
