use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::ContentError;
use crate::domain::models::{
    CreateContentInput, CreateReviewInput, UpdateContentInput,
    UploadMediaInput,
};
use crate::AppState;

// ─── Request / Response DTOs ────────────────────────────────────

#[derive(Deserialize)]
pub struct CreateContentRequest {
    pub language_id: Uuid,
    pub module_id: Uuid,
    pub lesson_id: Option<Uuid>,
    pub content_type: String,
    pub title: String,
    pub body: String,
    pub media_urls: Option<Vec<String>>,
    pub created_by: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateContentRequest {
    pub title: Option<String>,
    pub body: Option<String>,
    pub content_type: Option<String>,
    pub media_urls: Option<Vec<String>>,
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub struct ContentListQuery {
    pub language_id: Option<Uuid>,
    pub module_id: Option<Uuid>,
    pub status: Option<String>,
    pub content_type: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateReviewRequest {
    pub reviewer_id: Uuid,
    pub status: String,
    pub comments: Option<String>,
}

#[derive(Deserialize)]
pub struct UploadMediaRequest {
    pub filename: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub uploaded_by: Uuid,
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

pub async fn create_content(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateContentRequest>,
) -> impl IntoResponse {
    let input = CreateContentInput {
        language_id: req.language_id,
        module_id: req.module_id,
        lesson_id: req.lesson_id,
        content_type: req.content_type,
        title: req.title,
        body: req.body,
        media_urls: req.media_urls,
        created_by: req.created_by,
    };

    match state.content_service.create_content(input) {
        Ok(item) => (StatusCode::CREATED, Json(serde_json::json!(item)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn get_content(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.content_service.get_content(id) {
        Ok(item) => (StatusCode::OK, Json(serde_json::json!(item)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn update_content(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateContentRequest>,
) -> impl IntoResponse {
    let input = UpdateContentInput {
        title: req.title,
        body: req.body,
        content_type: req.content_type,
        media_urls: req.media_urls,
        status: req.status,
    };

    match state.content_service.update_content(id, input) {
        Ok(item) => (StatusCode::OK, Json(serde_json::json!(item)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn list_content(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ContentListQuery>,
) -> impl IntoResponse {
    match state.content_service.list_content(
        query.language_id,
        query.module_id,
        query.status.as_deref(),
        query.content_type.as_deref(),
    ) {
        Ok(items) => (StatusCode::OK, Json(serde_json::json!(items)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn create_review(
    State(state): State<Arc<AppState>>,
    Path(content_id): Path<Uuid>,
    Json(req): Json<CreateReviewRequest>,
) -> impl IntoResponse {
    let input = CreateReviewInput {
        content_item_id: content_id,
        reviewer_id: req.reviewer_id,
        status: req.status,
        comments: req.comments,
    };

    match state.content_service.create_review(input) {
        Ok(review) => (StatusCode::CREATED, Json(serde_json::json!(review)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn upload_media(
    State(state): State<Arc<AppState>>,
    Json(req): Json<UploadMediaRequest>,
) -> impl IntoResponse {
    let input = UploadMediaInput {
        filename: req.filename,
        content_type: req.content_type,
        size_bytes: req.size_bytes,
        uploaded_by: req.uploaded_by,
    };

    match state.content_service.upload_media(input) {
        Ok(asset) => (StatusCode::CREATED, Json(serde_json::json!(asset)))
            .into_response(),
        Err(e) => map_error(e).into_response(),
    }
}

pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "content-service",
    }))
}

// ─── Error Mapping ──────────────────────────────────────────────

fn map_error(err: ContentError) -> (StatusCode, Json<ErrorResponse>) {
    let (status, code) = match &err {
        ContentError::ContentNotFound(_) => {
            (StatusCode::NOT_FOUND, "NOT_FOUND")
        }
        ContentError::MediaNotFound(_) => {
            (StatusCode::NOT_FOUND, "MEDIA_NOT_FOUND")
        }
        ContentError::InvalidContentType(_) => {
            (StatusCode::BAD_REQUEST, "INVALID_CONTENT_TYPE")
        }
        ContentError::Unauthorized(_) => {
            (StatusCode::FORBIDDEN, "UNAUTHORIZED")
        }
        ContentError::ValidationError(_) => {
            (StatusCode::BAD_REQUEST, "VALIDATION_ERROR")
        }
        ContentError::RepositoryError(_) => {
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
