use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use super::schema::{assessments, credits, questions, student_answers};

/// Diesel model for reading assessments.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = assessments)]
pub struct AssessmentRow {
    pub id: Uuid,
    pub student_id: Uuid,
    pub module_id: Uuid,
    pub assessment_type: String,
    pub status: String,
    pub started_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub time_limit_minutes: Option<i32>,
}

/// Diesel model for inserting assessments.
#[derive(Insertable)]
#[diesel(table_name = assessments)]
pub struct NewAssessmentRow<'a> {
    pub id: Uuid,
    pub student_id: Uuid,
    pub module_id: Uuid,
    pub assessment_type: &'a str,
    pub status: &'a str,
    pub started_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub time_limit_minutes: Option<i32>,
}

/// Diesel model for reading questions.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = questions)]
pub struct QuestionRow {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub question_text: String,
    pub question_type: String,
    pub options: Option<String>,
    pub media_url: Option<String>,
    pub max_score: i32,
    pub correct_answer: String,
    pub order_index: i32,
}

/// Diesel model for reading student answers.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = student_answers)]
pub struct StudentAnswerRow {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub question_id: Uuid,
    pub answer: String,
    pub score: Option<i32>,
    pub is_correct: Option<bool>,
}

/// Diesel model for inserting student answers.
#[derive(Insertable)]
#[diesel(table_name = student_answers)]
pub struct NewStudentAnswerRow<'a> {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub question_id: Uuid,
    pub answer: &'a str,
    pub score: Option<i32>,
    pub is_correct: Option<bool>,
}

/// Diesel model for reading credits.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = credits)]
pub struct CreditRow {
    pub id: Uuid,
    pub student_id: Uuid,
    pub language_id: Uuid,
    pub grade: i32,
    pub semester: i32,
    pub score: f64,
    pub grade_letter: String,
    pub credit_points: f64,
    pub status: String,
    pub certificate_url: Option<String>,
    pub verification_code: String,
    pub issued_at: DateTime<Utc>,
}

/// Diesel model for inserting credits.
#[derive(Insertable)]
#[diesel(table_name = credits)]
pub struct NewCreditRow<'a> {
    pub id: Uuid,
    pub student_id: Uuid,
    pub language_id: Uuid,
    pub grade: i32,
    pub semester: i32,
    pub score: f64,
    pub grade_letter: &'a str,
    pub credit_points: f64,
    pub status: &'a str,
    pub certificate_url: Option<&'a str>,
    pub verification_code: &'a str,
    pub issued_at: DateTime<Utc>,
}
