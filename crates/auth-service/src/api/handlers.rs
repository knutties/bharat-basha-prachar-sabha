use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::AuthError;
use crate::domain::models::{
    RegisterStudentInput, RegisterTeacherInput, UpdateProfileInput,
};
use crate::AppState;
use shared::types::Grade;

// ─── Request / Response DTOs ────────────────────────────────────

#[derive(Deserialize)]
pub struct RegisterStudentRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub grade: u8,
    pub mother_tongue: String,
    pub state_of_residence: String,
    pub board: Option<String>,
    pub school_id: Option<Uuid>,
    pub parent_email: Option<String>,
}

#[derive(Deserialize)]
pub struct RegisterTeacherRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub languages: Vec<String>,
    pub qualifications: String,
    pub bio: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub grade: Option<u8>,
    pub state_of_residence: Option<String>,
    pub board: Option<String>,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub user_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user_id: Uuid,
    pub role: String,
    pub access_token: String,
    pub refresh_token: String,
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

pub async fn register_student(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterStudentRequest>,
) -> impl IntoResponse {
    let grade = match Grade::new(req.grade) {
        Some(g) => g,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: "VALIDATION_ERROR".to_string(),
                        message: "Grade must be between 1 and 12"
                            .to_string(),
                    },
                }),
            )
                .into_response();
        }
    };

    let input = RegisterStudentInput {
        name: req.name,
        email: req.email,
        password: req.password,
        phone: req.phone,
        grade,
        mother_tongue: req.mother_tongue,
        state_of_residence: req.state_of_residence,
        board: None, // TODO: parse from req.board
        school_id: None,
        parent_email: req.parent_email,
    };

    match state.auth_service.register_student(input) {
        Ok((user_id, tokens)) => (
            StatusCode::CREATED,
            Json(AuthResponse {
                user_id,
                access_token: tokens.access_token,
                refresh_token: tokens.refresh_token,
            }),
        )
            .into_response(),
        Err(e) => map_auth_error(e).into_response(),
    }
}

pub async fn register_teacher(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RegisterTeacherRequest>,
) -> impl IntoResponse {
    let input = RegisterTeacherInput {
        name: req.name,
        email: req.email,
        password: req.password,
        phone: req.phone,
        languages: req.languages,
        qualifications: req.qualifications,
        bio: req.bio,
    };

    match state.auth_service.register_teacher(input) {
        Ok((user_id, tokens)) => (
            StatusCode::CREATED,
            Json(AuthResponse {
                user_id,
                access_token: tokens.access_token,
                refresh_token: tokens.refresh_token,
            }),
        )
            .into_response(),
        Err(e) => map_auth_error(e).into_response(),
    }
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> impl IntoResponse {
    match state.auth_service.login(&req.email, &req.password) {
        Ok((user_id, role, tokens)) => (
            StatusCode::OK,
            Json(LoginResponse {
                user_id,
                role: format!("{:?}", role),
                access_token: tokens.access_token,
                refresh_token: tokens.refresh_token,
            }),
        )
            .into_response(),
        Err(e) => map_auth_error(e).into_response(),
    }
}

pub async fn refresh_token(
    State(_state): State<Arc<AppState>>,
    Json(_req): Json<RefreshTokenRequest>,
) -> impl IntoResponse {
    // TODO: Implement token refresh logic
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(MessageResponse {
            message: "Token refresh not yet implemented".to_string(),
        }),
    )
}

pub async fn get_profile(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.auth_service.get_profile(user_id) {
        Ok(user) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "user_id": user.id,
                "name": user.name,
                "email": user.email,
                "role": format!("{:?}", user.role),
                "phone": user.phone,
                "created_at": user.created_at.to_rfc3339(),
            })),
        )
            .into_response(),
        Err(e) => map_auth_error(e).into_response(),
    }
}

pub async fn update_profile(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<UpdateProfileRequest>,
) -> impl IntoResponse {
    let input = UpdateProfileInput {
        name: req.name,
        phone: req.phone,
        grade: req.grade.and_then(Grade::new),
        state_of_residence: req.state_of_residence,
        board: None, // TODO: parse from req.board
    };

    match state.auth_service.update_profile(user_id, input) {
        Ok(()) => (
            StatusCode::OK,
            Json(MessageResponse {
                message: "Profile updated successfully".to_string(),
            }),
        )
            .into_response(),
        Err(e) => map_auth_error(e).into_response(),
    }
}

pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "auth-service",
    }))
}

// ─── Error Mapping (API layer responsibility) ───────────────────

fn map_auth_error(err: AuthError) -> (StatusCode, Json<ErrorResponse>) {
    let (status, code) = match &err {
        AuthError::UserNotFound(_) => {
            (StatusCode::NOT_FOUND, "NOT_FOUND")
        }
        AuthError::EmailAlreadyExists(_) => {
            (StatusCode::CONFLICT, "CONFLICT")
        }
        AuthError::InvalidCredentials => {
            (StatusCode::UNAUTHORIZED, "INVALID_CREDENTIALS")
        }
        AuthError::InvalidToken(_) | AuthError::TokenExpired => {
            (StatusCode::UNAUTHORIZED, "INVALID_TOKEN")
        }
        AuthError::ValidationError(_) => {
            (StatusCode::BAD_REQUEST, "VALIDATION_ERROR")
        }
        AuthError::PasswordHashError(_) | AuthError::RepositoryError(_) => {
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
