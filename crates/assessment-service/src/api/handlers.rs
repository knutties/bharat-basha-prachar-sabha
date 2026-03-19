use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::AssessmentError;
use crate::domain::models::{CreateAssessmentInput, SubmitAssessmentInput};
use crate::AppState;

// ─── Request / Response DTOs ────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateAssessmentRequest {
    pub student_id: Uuid,
    pub module_id: Uuid,
    pub assessment_type: String,
    pub time_limit_minutes: Option<i32>,
}

#[derive(Deserialize)]
pub struct SubmitAssessmentRequest {
    pub answers: Vec<AnswerInput>,
}

#[derive(Deserialize)]
pub struct AnswerInput {
    pub question_id: Uuid,
    pub answer: String,
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

pub async fn create_assessment(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateAssessmentRequest>,
) -> impl IntoResponse {
    let input = CreateAssessmentInput {
        student_id: req.student_id,
        module_id: req.module_id,
        assessment_type: req.assessment_type,
        time_limit_minutes: req.time_limit_minutes,
    };

    match state.assessment_service.create_assessment(input) {
        Ok(assessment) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "id": assessment.id,
                "student_id": assessment.student_id,
                "module_id": assessment.module_id,
                "assessment_type": assessment.assessment_type,
                "status": assessment.status,
                "started_at": assessment.started_at,
                "time_limit_minutes": assessment.time_limit_minutes,
            })),
        )
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn submit_assessment(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<SubmitAssessmentRequest>,
) -> impl IntoResponse {
    let input = SubmitAssessmentInput {
        assessment_id: id,
        answers: req
            .answers
            .into_iter()
            .map(|a| crate::domain::models::AnswerSubmission {
                question_id: a.question_id,
                answer: a.answer,
            })
            .collect(),
    };

    match state.assessment_service.submit_assessment(input) {
        Ok(result) => (StatusCode::OK, Json(serde_json::json!(result)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn get_assessment_result(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.assessment_service.get_result(id) {
        Ok(result) => (StatusCode::OK, Json(serde_json::json!(result)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn get_student_credits(
    State(state): State<Arc<AppState>>,
    Path(student_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.assessment_service.get_student_credits(student_id) {
        Ok(credits) => (StatusCode::OK, Json(serde_json::json!(credits)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn verify_credential(
    State(state): State<Arc<AppState>>,
    Path(code): Path<String>,
) -> impl IntoResponse {
    match state.assessment_service.verify_credential(&code) {
        Ok(credit) => (StatusCode::OK, Json(serde_json::json!(credit)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "assessment-service",
    }))
}

// ─── Error Mapping ──────────────────────────────────────────────

fn map_error(err: AssessmentError) -> (StatusCode, Json<ErrorResponse>) {
    let (status, code) = match &err {
        AssessmentError::AssessmentNotFound(_) => {
            (StatusCode::NOT_FOUND, "NOT_FOUND")
        }
        AssessmentError::AlreadySubmitted(_) => {
            (StatusCode::CONFLICT, "ALREADY_SUBMITTED")
        }
        AssessmentError::TimeLimitExceeded(_) => {
            (StatusCode::BAD_REQUEST, "TIME_LIMIT_EXCEEDED")
        }
        AssessmentError::CreditNotFound(_) => {
            (StatusCode::NOT_FOUND, "CREDIT_NOT_FOUND")
        }
        AssessmentError::InvalidVerificationCode(_) => {
            (StatusCode::NOT_FOUND, "INVALID_VERIFICATION_CODE")
        }
        AssessmentError::ValidationError(_) => {
            (StatusCode::BAD_REQUEST, "VALIDATION_ERROR")
        }
        AssessmentError::RepositoryError(_) => {
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
