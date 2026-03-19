use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

/// An assessment taken by a student for a specific module.
#[derive(Debug, Clone, Serialize)]
pub struct Assessment {
    pub id: Uuid,
    pub student_id: Uuid,
    pub module_id: Uuid,
    pub assessment_type: String,
    pub status: String,
    pub started_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub time_limit_minutes: Option<i32>,
}

/// A question belonging to an assessment.
#[derive(Debug, Clone, Serialize)]
pub struct Question {
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

/// A student's answer to a specific question.
#[derive(Debug, Clone, Serialize)]
pub struct StudentAnswer {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub question_id: Uuid,
    pub answer: String,
    pub score: Option<i32>,
    pub is_correct: Option<bool>,
}

/// A credit/grade earned by a student for a language.
#[derive(Debug, Clone, Serialize)]
pub struct Credit {
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

/// Input for creating a new assessment.
#[derive(Debug)]
pub struct CreateAssessmentInput {
    pub student_id: Uuid,
    pub module_id: Uuid,
    pub assessment_type: String,
    pub time_limit_minutes: Option<i32>,
}

/// Input for submitting an assessment.
#[derive(Debug)]
pub struct SubmitAssessmentInput {
    pub assessment_id: Uuid,
    pub answers: Vec<AnswerSubmission>,
}

/// A single answer submission.
#[derive(Debug)]
pub struct AnswerSubmission {
    pub question_id: Uuid,
    pub answer: String,
}

/// Result returned after assessment submission.
#[derive(Debug, Clone, Serialize)]
pub struct AssessmentResult {
    pub assessment_id: Uuid,
    pub total_score: i32,
    pub max_score: i32,
    pub percentage: f64,
    pub passed: bool,
}
