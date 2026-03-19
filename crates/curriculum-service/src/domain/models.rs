use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::types::{Grade, ProficiencyLevel, SkillType};
use uuid::Uuid;

// ─── Lesson Type ────────────────────────────────────────────────

/// The type/format of a lesson.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LessonType {
    Video,
    Text,
    Interactive,
    Exercise,
    Cultural,
}

impl LessonType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LessonType::Video => "VIDEO",
            LessonType::Text => "TEXT",
            LessonType::Interactive => "INTERACTIVE",
            LessonType::Exercise => "EXERCISE",
            LessonType::Cultural => "CULTURAL",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "VIDEO" => Some(LessonType::Video),
            "TEXT" => Some(LessonType::Text),
            "INTERACTIVE" => Some(LessonType::Interactive),
            "EXERCISE" => Some(LessonType::Exercise),
            "CULTURAL" => Some(LessonType::Cultural),
            _ => None,
        }
    }
}

// ─── Core Domain Models ─────────────────────────────────────────

/// A language available on the platform.
#[derive(Debug, Clone)]
pub struct Language {
    pub id: Uuid,
    pub name: String,
    pub script: String,
    pub iso_code: String,
    pub family: String,
}

/// A curriculum for a specific language and grade.
#[derive(Debug, Clone)]
pub struct Curriculum {
    pub id: Uuid,
    pub language_id: Uuid,
    pub grade: Grade,
    pub title: String,
    pub description: Option<String>,
}

/// A module within a curriculum, focusing on a specific skill.
#[derive(Debug, Clone, Serialize)]
pub struct Module {
    pub id: Uuid,
    pub curriculum_id: Uuid,
    pub title: String,
    pub description: String,
    pub skill_type: SkillType,
    pub order_index: i32,
    pub estimated_minutes: i32,
}

/// A lesson within a module.
#[derive(Debug, Clone)]
pub struct Lesson {
    pub id: Uuid,
    pub module_id: Uuid,
    pub title: String,
    pub description: String,
    pub lesson_type: LessonType,
    pub content: String,
    pub media_urls: Vec<String>,
    pub duration_minutes: i32,
    pub order_index: i32,
}

/// A student's enrollment in a language course.
#[derive(Debug, Clone)]
pub struct Enrollment {
    pub id: Uuid,
    pub student_id: Uuid,
    pub language_id: Uuid,
    pub grade: Grade,
    pub proficiency: ProficiencyLevel,
    pub enrolled_at: DateTime<Utc>,
}

/// A record of a student's progress on a specific lesson.
#[derive(Debug, Clone)]
pub struct Progress {
    pub id: Uuid,
    pub student_id: Uuid,
    pub lesson_id: Uuid,
    pub completed_at: DateTime<Utc>,
    pub score: Option<f64>,
    pub time_spent_seconds: i32,
}

// ─── Computed / Composite Models ────────────────────────────────

/// A computed view of a student's learning path for an enrollment.
#[derive(Debug, Clone, Serialize)]
pub struct LearningPath {
    pub enrollment_id: Uuid,
    pub language_name: String,
    pub current_module: Module,
    pub completed_modules: i32,
    pub total_modules: i32,
    pub overall_progress: f64,
    pub next_modules: Vec<Module>,
}

// ─── Input Structs ──────────────────────────────────────────────

/// Input for enrolling a student in a language.
#[derive(Debug)]
pub struct EnrollInput {
    pub student_id: Uuid,
    pub language_id: Uuid,
    pub grade: Grade,
    pub initial_proficiency: Option<ProficiencyLevel>,
}

/// Input for recording lesson progress.
#[derive(Debug)]
pub struct RecordProgressInput {
    pub student_id: Uuid,
    pub lesson_id: Uuid,
    pub completed_at: DateTime<Utc>,
    pub score: Option<f64>,
    pub time_spent_seconds: i32,
}
