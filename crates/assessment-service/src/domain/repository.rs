use uuid::Uuid;

use super::errors::AssessmentError;
use super::models::{Assessment, Credit, Question, StudentAnswer};

/// Repository trait for assessment persistence.
/// Implemented by the database layer; consumed by domain services.
#[cfg_attr(test, mockall::automock)]
pub trait AssessmentRepository: Send + Sync {
    /// Create a new assessment.
    fn create_assessment(
        &self,
        assessment: &Assessment,
    ) -> Result<Assessment, AssessmentError>;

    /// Find an assessment by ID.
    fn find_assessment_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Assessment>, AssessmentError>;

    /// Update an assessment (e.g., mark as submitted).
    fn update_assessment(
        &self,
        assessment: &Assessment,
    ) -> Result<Assessment, AssessmentError>;

    /// Get questions for an assessment.
    fn find_questions_by_assessment(
        &self,
        assessment_id: Uuid,
    ) -> Result<Vec<Question>, AssessmentError>;

    /// Save student answers.
    fn save_student_answers(
        &self,
        answers: &[StudentAnswer],
    ) -> Result<(), AssessmentError>;

    /// Get student answers for an assessment.
    fn find_answers_by_assessment(
        &self,
        assessment_id: Uuid,
    ) -> Result<Vec<StudentAnswer>, AssessmentError>;

    /// Get all credits for a student.
    fn find_credits_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<Credit>, AssessmentError>;

    /// Find a credit by verification code.
    fn find_credit_by_verification_code(
        &self,
        code: &str,
    ) -> Result<Option<Credit>, AssessmentError>;

    /// Create a new credit record.
    fn create_credit(
        &self,
        credit: &Credit,
    ) -> Result<Credit, AssessmentError>;
}
