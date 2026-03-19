use chrono::Utc;
use uuid::Uuid;

use super::errors::AssessmentError;
use super::models::{
    Assessment, AssessmentResult, CreateAssessmentInput, Credit,
    StudentAnswer, SubmitAssessmentInput,
};
use super::repository::AssessmentRepository;

/// Core assessment service containing all business logic.
pub struct AssessmentServiceImpl<R: AssessmentRepository> {
    repo: R,
}

impl<R: AssessmentRepository> AssessmentServiceImpl<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// Create a new assessment for a student.
    pub fn create_assessment(
        &self,
        input: CreateAssessmentInput,
    ) -> Result<Assessment, AssessmentError> {
        let assessment = Assessment {
            id: Uuid::new_v4(),
            student_id: input.student_id,
            module_id: input.module_id,
            assessment_type: input.assessment_type,
            status: "in_progress".to_string(),
            started_at: Utc::now(),
            submitted_at: None,
            time_limit_minutes: input.time_limit_minutes,
        };

        self.repo.create_assessment(&assessment)
    }

    /// Submit answers for an assessment and compute the result.
    pub fn submit_assessment(
        &self,
        input: SubmitAssessmentInput,
    ) -> Result<AssessmentResult, AssessmentError> {
        let assessment = self
            .repo
            .find_assessment_by_id(input.assessment_id)?
            .ok_or_else(|| {
                AssessmentError::AssessmentNotFound(
                    input.assessment_id.to_string(),
                )
            })?;

        if assessment.status == "submitted" {
            return Err(AssessmentError::AlreadySubmitted(
                assessment.id.to_string(),
            ));
        }

        // Check time limit
        if let Some(limit) = assessment.time_limit_minutes {
            let elapsed = Utc::now()
                .signed_duration_since(assessment.started_at)
                .num_minutes();
            if elapsed > limit as i64 {
                return Err(AssessmentError::TimeLimitExceeded(
                    assessment.id.to_string(),
                ));
            }
        }

        // Get questions for grading
        let questions =
            self.repo.find_questions_by_assessment(assessment.id)?;

        let mut total_score = 0i32;
        let mut max_score = 0i32;
        let mut student_answers = Vec::new();

        for submission in &input.answers {
            if let Some(question) =
                questions.iter().find(|q| q.id == submission.question_id)
            {
                max_score += question.max_score;
                let is_correct = submission.answer == question.correct_answer;
                let score = if is_correct { question.max_score } else { 0 };
                total_score += score;

                student_answers.push(StudentAnswer {
                    id: Uuid::new_v4(),
                    assessment_id: assessment.id,
                    question_id: submission.question_id,
                    answer: submission.answer.clone(),
                    score: Some(score),
                    is_correct: Some(is_correct),
                });
            }
        }

        self.repo.save_student_answers(&student_answers)?;

        // Mark assessment as submitted
        let mut updated = assessment;
        updated.status = "submitted".to_string();
        updated.submitted_at = Some(Utc::now());
        self.repo.update_assessment(&updated)?;

        let percentage = if max_score > 0 {
            (total_score as f64 / max_score as f64) * 100.0
        } else {
            0.0
        };

        Ok(AssessmentResult {
            assessment_id: updated.id,
            total_score,
            max_score,
            percentage,
            passed: percentage >= 40.0,
        })
    }

    /// Get the result of a completed assessment.
    pub fn get_result(
        &self,
        assessment_id: Uuid,
    ) -> Result<AssessmentResult, AssessmentError> {
        let assessment = self
            .repo
            .find_assessment_by_id(assessment_id)?
            .ok_or_else(|| {
                AssessmentError::AssessmentNotFound(
                    assessment_id.to_string(),
                )
            })?;

        let questions =
            self.repo.find_questions_by_assessment(assessment_id)?;
        let answers =
            self.repo.find_answers_by_assessment(assessment_id)?;

        let max_score: i32 = questions.iter().map(|q| q.max_score).sum();
        let total_score: i32 =
            answers.iter().filter_map(|a| a.score).sum();
        let percentage = if max_score > 0 {
            (total_score as f64 / max_score as f64) * 100.0
        } else {
            0.0
        };

        Ok(AssessmentResult {
            assessment_id: assessment.id,
            total_score,
            max_score,
            percentage,
            passed: percentage >= 40.0,
        })
    }

    /// Get all credits earned by a student.
    pub fn get_student_credits(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<Credit>, AssessmentError> {
        self.repo.find_credits_by_student(student_id)
    }

    /// Verify a credential by its verification code.
    pub fn verify_credential(
        &self,
        code: &str,
    ) -> Result<Credit, AssessmentError> {
        self.repo
            .find_credit_by_verification_code(code)?
            .ok_or_else(|| {
                AssessmentError::InvalidVerificationCode(code.to_string())
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repository::MockAssessmentRepository;

    fn make_service() -> AssessmentServiceImpl<MockAssessmentRepository> {
        let mock_repo = MockAssessmentRepository::new();
        AssessmentServiceImpl::new(mock_repo)
    }

    #[test]
    fn test_create_assessment() {
        let mut mock_repo = MockAssessmentRepository::new();
        mock_repo
            .expect_create_assessment()
            .returning(|a| Ok(a.clone()));

        let service = AssessmentServiceImpl::new(mock_repo);
        let input = CreateAssessmentInput {
            student_id: Uuid::new_v4(),
            module_id: Uuid::new_v4(),
            assessment_type: "quiz".to_string(),
            time_limit_minutes: Some(30),
        };

        let result = service.create_assessment(input);
        assert!(result.is_ok());
        let assessment = result.unwrap();
        assert_eq!(assessment.status, "in_progress");
    }
}
