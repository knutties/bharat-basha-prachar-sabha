use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use super::schema::{curricula, enrollments, languages, lessons, modules, progress};

// ─── Languages ──────────────────────────────────────────────────

/// Diesel model for reading languages from the database.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = languages)]
pub struct LanguageRow {
    pub id: Uuid,
    pub name: String,
    pub script: String,
    pub iso_code: String,
    pub family: String,
}

/// Diesel model for inserting new languages.
#[derive(Insertable)]
#[diesel(table_name = languages)]
pub struct NewLanguageRow<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub script: &'a str,
    pub iso_code: &'a str,
    pub family: &'a str,
}

// ─── Curricula ──────────────────────────────────────────────────

/// Diesel model for reading curricula from the database.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = curricula)]
pub struct CurriculumRow {
    pub id: Uuid,
    pub language_id: Uuid,
    pub grade: i16,
    pub title: String,
    pub description: Option<String>,
}

/// Diesel model for inserting new curricula.
#[derive(Insertable)]
#[diesel(table_name = curricula)]
pub struct NewCurriculumRow<'a> {
    pub id: Uuid,
    pub language_id: Uuid,
    pub grade: i16,
    pub title: &'a str,
    pub description: Option<&'a str>,
}

// ─── Modules ────────────────────────────────────────────────────

/// Diesel model for reading modules from the database.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = modules)]
pub struct ModuleRow {
    pub id: Uuid,
    pub curriculum_id: Uuid,
    pub title: String,
    pub description: String,
    pub skill_type: String,
    pub order_index: i32,
    pub estimated_minutes: i32,
}

/// Diesel model for inserting new modules.
#[derive(Insertable)]
#[diesel(table_name = modules)]
pub struct NewModuleRow<'a> {
    pub id: Uuid,
    pub curriculum_id: Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub skill_type: &'a str,
    pub order_index: i32,
    pub estimated_minutes: i32,
}

// ─── Lessons ────────────────────────────────────────────────────

/// Diesel model for reading lessons from the database.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = lessons)]
pub struct LessonRow {
    pub id: Uuid,
    pub module_id: Uuid,
    pub title: String,
    pub description: String,
    pub lesson_type: String,
    pub content: String,
    pub media_urls: Vec<String>,
    pub duration_minutes: i32,
    pub order_index: i32,
}

/// Diesel model for inserting new lessons.
#[derive(Insertable)]
#[diesel(table_name = lessons)]
pub struct NewLessonRow<'a> {
    pub id: Uuid,
    pub module_id: Uuid,
    pub title: &'a str,
    pub description: &'a str,
    pub lesson_type: &'a str,
    pub content: &'a str,
    pub media_urls: &'a [String],
    pub duration_minutes: i32,
    pub order_index: i32,
}

// ─── Enrollments ────────────────────────────────────────────────

/// Diesel model for reading enrollments from the database.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = enrollments)]
pub struct EnrollmentRow {
    pub id: Uuid,
    pub student_id: Uuid,
    pub language_id: Uuid,
    pub grade: i16,
    pub proficiency: String,
    pub enrolled_at: DateTime<Utc>,
}

/// Diesel model for inserting new enrollments.
#[derive(Insertable)]
#[diesel(table_name = enrollments)]
pub struct NewEnrollmentRow<'a> {
    pub id: Uuid,
    pub student_id: Uuid,
    pub language_id: Uuid,
    pub grade: i16,
    pub proficiency: &'a str,
    pub enrolled_at: DateTime<Utc>,
}

// ─── Progress ───────────────────────────────────────────────────

/// Diesel model for reading progress records from the database.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = progress)]
pub struct ProgressRow {
    pub id: Uuid,
    pub student_id: Uuid,
    pub lesson_id: Uuid,
    pub completed_at: DateTime<Utc>,
    pub score: Option<f64>,
    pub time_spent_seconds: i32,
}

/// Diesel model for inserting new progress records.
#[derive(Insertable)]
#[diesel(table_name = progress)]
pub struct NewProgressRow {
    pub id: Uuid,
    pub student_id: Uuid,
    pub lesson_id: Uuid,
    pub completed_at: DateTime<Utc>,
    pub score: Option<f64>,
    pub time_spent_seconds: i32,
}
