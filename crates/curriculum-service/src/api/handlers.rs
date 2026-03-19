use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::errors::CurriculumError;
use crate::domain::models::{EnrollInput, RecordProgressInput};
use crate::AppState;
use shared::types::Grade;

// ─── Request / Response DTOs ────────────────────────────────────

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct LanguageResponse {
    pub id: Uuid,
    pub name: String,
    pub script: String,
    pub iso_code: String,
    pub family: String,
}

#[derive(Serialize)]
pub struct LanguageListResponse {
    pub languages: Vec<LanguageResponse>,
}

#[derive(Serialize)]
pub struct ModuleSummaryResponse {
    pub id: Uuid,
    pub title: String,
    pub skill_type: String,
    pub order_index: i32,
    pub estimated_minutes: i32,
}

#[derive(Serialize)]
pub struct CurriculumResponse {
    pub id: Uuid,
    pub language_id: Uuid,
    pub grade: u8,
    pub title: String,
    pub description: Option<String>,
    pub modules: Vec<ModuleSummaryResponse>,
}

#[derive(Serialize)]
pub struct LessonSummaryResponse {
    pub id: Uuid,
    pub title: String,
    pub lesson_type: String,
    pub order_index: i32,
    pub duration_minutes: i32,
}

#[derive(Serialize)]
pub struct ModuleDetailResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub skill_type: String,
    pub order_index: i32,
    pub estimated_minutes: i32,
    pub lessons: Vec<LessonSummaryResponse>,
}

#[derive(Serialize)]
pub struct LessonDetailResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub lesson_type: String,
    pub content: String,
    pub media_urls: Vec<String>,
    pub duration_minutes: i32,
}

#[derive(Deserialize)]
pub struct CurriculumQuery {
    pub grade: Option<u8>,
}

#[derive(Deserialize)]
pub struct ModulesQuery {
    pub grade: Option<u8>,
    pub skill_type: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Deserialize)]
pub struct EnrollRequest {
    pub student_id: Uuid,
    pub language_id: Uuid,
    pub grade: u8,
    pub initial_proficiency: Option<String>,
}

#[derive(Serialize)]
pub struct EnrollResponse {
    pub enrollment_id: Uuid,
    pub message: String,
}

#[derive(Deserialize)]
pub struct RecordProgressRequest {
    pub student_id: Uuid,
    pub lesson_id: Uuid,
    pub completed_at: String,
    pub score: Option<f64>,
    pub time_spent_seconds: i32,
}

#[derive(Serialize)]
pub struct RecordProgressResponse {
    pub progress_id: Uuid,
    pub message: String,
    pub module_completed: Option<bool>,
}

#[derive(Serialize)]
pub struct LearningPathResponse {
    pub enrollment_id: Uuid,
    pub language_name: String,
    pub current_module: ModuleSummaryResponse,
    pub completed_modules: i32,
    pub total_modules: i32,
    pub overall_progress: f64,
    pub next_modules: Vec<ModuleSummaryResponse>,
}

#[derive(Serialize)]
pub struct ModulesListResponse {
    pub modules: Vec<ModuleSummaryResponse>,
    pub pagination: PaginationResponse,
}

#[derive(Serialize)]
pub struct PaginationResponse {
    pub page: u64,
    pub page_size: u64,
    pub total_items: u64,
    pub total_pages: u64,
}

// ─── Handlers ───────────────────────────────────────────────────

pub async fn list_languages(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match state.curriculum_service.list_languages() {
        Ok(languages) => {
            let response = LanguageListResponse {
                languages: languages
                    .into_iter()
                    .map(|l| LanguageResponse {
                        id: l.id,
                        name: l.name,
                        script: l.script,
                        iso_code: l.iso_code,
                        family: l.family,
                    })
                    .collect(),
            };
            (StatusCode::OK, Json(serde_json::to_value(response).unwrap()))
                .into_response()
        }
        Err(e) => map_curriculum_error(e).into_response(),
    }
}

pub async fn get_curriculum(
    State(state): State<Arc<AppState>>,
    Path(language_id): Path<Uuid>,
    Query(query): Query<CurriculumQuery>,
) -> impl IntoResponse {
    let grade = query.grade.and_then(Grade::new);

    match state.curriculum_service.get_curriculum(language_id, grade) {
        Ok((curriculum, modules)) => {
            let response = CurriculumResponse {
                id: curriculum.id,
                language_id: curriculum.language_id,
                grade: curriculum.grade.value(),
                title: curriculum.title,
                description: curriculum.description,
                modules: modules
                    .into_iter()
                    .map(module_to_summary)
                    .collect(),
            };
            (StatusCode::OK, Json(serde_json::to_value(response).unwrap()))
                .into_response()
        }
        Err(e) => map_curriculum_error(e).into_response(),
    }
}

pub async fn list_modules(
    State(state): State<Arc<AppState>>,
    Path(language_id): Path<Uuid>,
    Query(query): Query<ModulesQuery>,
) -> impl IntoResponse {
    let grade = query.grade.and_then(Grade::new);
    let skill_type = query.skill_type.as_deref().and_then(parse_skill_type);

    match state
        .curriculum_service
        .list_modules(language_id, grade, skill_type)
    {
        Ok(modules) => {
            let page = query.page.unwrap_or(1);
            let page_size = query.page_size.unwrap_or(20);
            let total_items = modules.len() as u64;
            let total_pages =
                (total_items + page_size - 1) / page_size.max(1);

            let offset = (page.saturating_sub(1)) * page_size;
            let paged: Vec<_> = modules
                .into_iter()
                .skip(offset as usize)
                .take(page_size as usize)
                .map(module_to_summary)
                .collect();

            let response = ModulesListResponse {
                modules: paged,
                pagination: PaginationResponse {
                    page,
                    page_size,
                    total_items,
                    total_pages,
                },
            };
            (StatusCode::OK, Json(serde_json::to_value(response).unwrap()))
                .into_response()
        }
        Err(e) => map_curriculum_error(e).into_response(),
    }
}

pub async fn get_module(
    State(state): State<Arc<AppState>>,
    Path(module_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.curriculum_service.get_module(module_id) {
        Ok((module, lessons)) => {
            let response = ModuleDetailResponse {
                id: module.id,
                title: module.title,
                description: module.description,
                skill_type: format!("{:?}", module.skill_type),
                order_index: module.order_index,
                estimated_minutes: module.estimated_minutes,
                lessons: lessons
                    .into_iter()
                    .map(|l| LessonSummaryResponse {
                        id: l.id,
                        title: l.title,
                        lesson_type: l.lesson_type.as_str().to_string(),
                        order_index: l.order_index,
                        duration_minutes: l.duration_minutes,
                    })
                    .collect(),
            };
            (StatusCode::OK, Json(serde_json::to_value(response).unwrap()))
                .into_response()
        }
        Err(e) => map_curriculum_error(e).into_response(),
    }
}

pub async fn get_lesson(
    State(state): State<Arc<AppState>>,
    Path(lesson_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.curriculum_service.get_lesson(lesson_id) {
        Ok(lesson) => {
            let response = LessonDetailResponse {
                id: lesson.id,
                title: lesson.title,
                description: lesson.description,
                lesson_type: lesson.lesson_type.as_str().to_string(),
                content: lesson.content,
                media_urls: lesson.media_urls,
                duration_minutes: lesson.duration_minutes,
            };
            (StatusCode::OK, Json(serde_json::to_value(response).unwrap()))
                .into_response()
        }
        Err(e) => map_curriculum_error(e).into_response(),
    }
}

pub async fn enroll_in_language(
    State(state): State<Arc<AppState>>,
    Json(req): Json<EnrollRequest>,
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

    let initial_proficiency =
        req.initial_proficiency.as_deref().and_then(parse_proficiency);

    let input = EnrollInput {
        student_id: req.student_id,
        language_id: req.language_id,
        grade,
        initial_proficiency,
    };

    match state.curriculum_service.enroll(input) {
        Ok(enrollment) => (
            StatusCode::CREATED,
            Json(EnrollResponse {
                enrollment_id: enrollment.id,
                message: "Successfully enrolled".to_string(),
            }),
        )
            .into_response(),
        Err(e) => map_curriculum_error(e).into_response(),
    }
}

pub async fn get_learning_path(
    State(state): State<Arc<AppState>>,
    Path(enrollment_id): Path<Uuid>,
) -> impl IntoResponse {
    match state.curriculum_service.get_learning_path(enrollment_id) {
        Ok(path) => {
            let response = LearningPathResponse {
                enrollment_id: path.enrollment_id,
                language_name: path.language_name,
                current_module: module_to_summary(path.current_module),
                completed_modules: path.completed_modules,
                total_modules: path.total_modules,
                overall_progress: path.overall_progress,
                next_modules: path
                    .next_modules
                    .into_iter()
                    .map(module_to_summary)
                    .collect(),
            };
            (StatusCode::OK, Json(serde_json::to_value(response).unwrap()))
                .into_response()
        }
        Err(e) => map_curriculum_error(e).into_response(),
    }
}

pub async fn record_progress(
    State(state): State<Arc<AppState>>,
    Json(req): Json<RecordProgressRequest>,
) -> impl IntoResponse {
    let completed_at = match req.completed_at.parse::<chrono::DateTime<chrono::Utc>>() {
        Ok(dt) => dt,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: "VALIDATION_ERROR".to_string(),
                        message: "Invalid completed_at timestamp format"
                            .to_string(),
                    },
                }),
            )
                .into_response();
        }
    };

    let input = RecordProgressInput {
        student_id: req.student_id,
        lesson_id: req.lesson_id,
        completed_at,
        score: req.score,
        time_spent_seconds: req.time_spent_seconds,
    };

    match state.curriculum_service.record_progress(input) {
        Ok((progress, module_completed)) => (
            StatusCode::CREATED,
            Json(RecordProgressResponse {
                progress_id: progress.id,
                message: "Progress recorded successfully".to_string(),
                module_completed: Some(module_completed),
            }),
        )
            .into_response(),
        Err(e) => map_curriculum_error(e).into_response(),
    }
}

pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "curriculum-service",
    }))
}

// ─── Helper Functions ───────────────────────────────────────────

fn module_to_summary(
    m: crate::domain::models::Module,
) -> ModuleSummaryResponse {
    ModuleSummaryResponse {
        id: m.id,
        title: m.title,
        skill_type: format!("{:?}", m.skill_type),
        order_index: m.order_index,
        estimated_minutes: m.estimated_minutes,
    }
}

fn parse_skill_type(s: &str) -> Option<shared::types::SkillType> {
    use shared::types::SkillType;
    match s.to_lowercase().as_str() {
        "reading" => Some(SkillType::Reading),
        "writing" => Some(SkillType::Writing),
        "listening" => Some(SkillType::Listening),
        "speaking" => Some(SkillType::Speaking),
        _ => None,
    }
}

fn parse_proficiency(s: &str) -> Option<shared::types::ProficiencyLevel> {
    use shared::types::ProficiencyLevel;
    match s.to_lowercase().as_str() {
        "beginner" => Some(ProficiencyLevel::Beginner),
        "intermediate" => Some(ProficiencyLevel::Intermediate),
        "advanced" => Some(ProficiencyLevel::Advanced),
        _ => None,
    }
}

// ─── Error Mapping (API layer responsibility) ───────────────────

fn map_curriculum_error(
    err: CurriculumError,
) -> (StatusCode, Json<ErrorResponse>) {
    let (status, code) = match &err {
        CurriculumError::LanguageNotFound(_)
        | CurriculumError::CurriculumNotFound(_)
        | CurriculumError::ModuleNotFound(_)
        | CurriculumError::LessonNotFound(_)
        | CurriculumError::EnrollmentNotFound(_) => {
            (StatusCode::NOT_FOUND, "NOT_FOUND")
        }
        CurriculumError::AlreadyEnrolled(_) => {
            (StatusCode::CONFLICT, "CONFLICT")
        }
        CurriculumError::ValidationError(_) => {
            (StatusCode::BAD_REQUEST, "VALIDATION_ERROR")
        }
        CurriculumError::RepositoryError(_) => {
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
