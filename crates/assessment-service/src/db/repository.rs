use diesel::prelude::*;
use uuid::Uuid;

use super::connection::DbPool;
use super::models::*;
use super::schema::{assessments, credits, questions, student_answers};
use crate::domain::errors::AssessmentError;
use crate::domain::models::{Assessment, Credit, Question, StudentAnswer};
use crate::domain::repository::AssessmentRepository;

/// PostgreSQL implementation of the AssessmentRepository trait.
pub struct PgAssessmentRepository {
    pool: DbPool,
}

impl PgAssessmentRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn get_conn(
        &self,
    ) -> Result<
        diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<PgConnection>,
        >,
        AssessmentError,
    > {
        self.pool
            .get()
            .map_err(|e| AssessmentError::RepositoryError(e.to_string()))
    }
}

impl AssessmentRepository for PgAssessmentRepository {
    fn create_assessment(
        &self,
        assessment: &Assessment,
    ) -> Result<Assessment, AssessmentError> {
        let mut conn = self.get_conn()?;

        let new_row = NewAssessmentRow {
            id: assessment.id,
            student_id: assessment.student_id,
            module_id: assessment.module_id,
            assessment_type: &assessment.assessment_type,
            status: &assessment.status,
            started_at: assessment.started_at,
            submitted_at: assessment.submitted_at,
            time_limit_minutes: assessment.time_limit_minutes,
        };

        let row = diesel::insert_into(assessments::table)
            .values(&new_row)
            .returning(AssessmentRow::as_returning())
            .get_result::<AssessmentRow>(&mut conn)
            .map_err(|e| AssessmentError::RepositoryError(e.to_string()))?;

        Ok(row_to_assessment(row))
    }

    fn find_assessment_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Assessment>, AssessmentError> {
        let mut conn = self.get_conn()?;

        let row = assessments::table
            .filter(assessments::id.eq(id))
            .select(AssessmentRow::as_select())
            .first::<AssessmentRow>(&mut conn)
            .optional()
            .map_err(|e| AssessmentError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_assessment))
    }

    fn update_assessment(
        &self,
        assessment: &Assessment,
    ) -> Result<Assessment, AssessmentError> {
        let mut conn = self.get_conn()?;

        diesel::update(assessments::table.filter(assessments::id.eq(assessment.id)))
            .set((
                assessments::status.eq(&assessment.status),
                assessments::submitted_at.eq(assessment.submitted_at),
            ))
            .execute(&mut conn)
            .map_err(|e| AssessmentError::RepositoryError(e.to_string()))?;

        self.find_assessment_by_id(assessment.id)?
            .ok_or_else(|| {
                AssessmentError::AssessmentNotFound(assessment.id.to_string())
            })
    }

    fn find_questions_by_assessment(
        &self,
        assessment_id: Uuid,
    ) -> Result<Vec<Question>, AssessmentError> {
        let mut conn = self.get_conn()?;

        let rows = questions::table
            .filter(questions::assessment_id.eq(assessment_id))
            .order(questions::order_index.asc())
            .select(QuestionRow::as_select())
            .load::<QuestionRow>(&mut conn)
            .map_err(|e| AssessmentError::RepositoryError(e.to_string()))?;

        Ok(rows.into_iter().map(row_to_question).collect())
    }

    fn save_student_answers(
        &self,
        answers: &[StudentAnswer],
    ) -> Result<(), AssessmentError> {
        let mut conn = self.get_conn()?;

        let new_rows: Vec<NewStudentAnswerRow> = answers
            .iter()
            .map(|a| NewStudentAnswerRow {
                id: a.id,
                assessment_id: a.assessment_id,
                question_id: a.question_id,
                answer: &a.answer,
                score: a.score,
                is_correct: a.is_correct,
            })
            .collect();

        diesel::insert_into(student_answers::table)
            .values(&new_rows)
            .execute(&mut conn)
            .map_err(|e| AssessmentError::RepositoryError(e.to_string()))?;

        Ok(())
    }

    fn find_answers_by_assessment(
        &self,
        assessment_id: Uuid,
    ) -> Result<Vec<StudentAnswer>, AssessmentError> {
        let mut conn = self.get_conn()?;

        let rows = student_answers::table
            .filter(student_answers::assessment_id.eq(assessment_id))
            .select(StudentAnswerRow::as_select())
            .load::<StudentAnswerRow>(&mut conn)
            .map_err(|e| AssessmentError::RepositoryError(e.to_string()))?;

        Ok(rows.into_iter().map(row_to_student_answer).collect())
    }

    fn find_credits_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<Credit>, AssessmentError> {
        let mut conn = self.get_conn()?;

        let rows = credits::table
            .filter(credits::student_id.eq(student_id))
            .select(CreditRow::as_select())
            .load::<CreditRow>(&mut conn)
            .map_err(|e| AssessmentError::RepositoryError(e.to_string()))?;

        Ok(rows.into_iter().map(row_to_credit).collect())
    }

    fn find_credit_by_verification_code(
        &self,
        code: &str,
    ) -> Result<Option<Credit>, AssessmentError> {
        let mut conn = self.get_conn()?;

        let row = credits::table
            .filter(credits::verification_code.eq(code))
            .select(CreditRow::as_select())
            .first::<CreditRow>(&mut conn)
            .optional()
            .map_err(|e| AssessmentError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_credit))
    }

    fn create_credit(
        &self,
        credit: &Credit,
    ) -> Result<Credit, AssessmentError> {
        let mut conn = self.get_conn()?;

        let new_row = NewCreditRow {
            id: credit.id,
            student_id: credit.student_id,
            language_id: credit.language_id,
            grade: credit.grade,
            semester: credit.semester,
            score: credit.score,
            grade_letter: &credit.grade_letter,
            credit_points: credit.credit_points,
            status: &credit.status,
            certificate_url: credit.certificate_url.as_deref(),
            verification_code: &credit.verification_code,
            issued_at: credit.issued_at,
        };

        let row = diesel::insert_into(credits::table)
            .values(&new_row)
            .returning(CreditRow::as_returning())
            .get_result::<CreditRow>(&mut conn)
            .map_err(|e| AssessmentError::RepositoryError(e.to_string()))?;

        Ok(row_to_credit(row))
    }
}

fn row_to_assessment(row: AssessmentRow) -> Assessment {
    Assessment {
        id: row.id,
        student_id: row.student_id,
        module_id: row.module_id,
        assessment_type: row.assessment_type,
        status: row.status,
        started_at: row.started_at,
        submitted_at: row.submitted_at,
        time_limit_minutes: row.time_limit_minutes,
    }
}

fn row_to_question(row: QuestionRow) -> Question {
    Question {
        id: row.id,
        assessment_id: row.assessment_id,
        question_text: row.question_text,
        question_type: row.question_type,
        options: row.options,
        media_url: row.media_url,
        max_score: row.max_score,
        correct_answer: row.correct_answer,
        order_index: row.order_index,
    }
}

fn row_to_student_answer(row: StudentAnswerRow) -> StudentAnswer {
    StudentAnswer {
        id: row.id,
        assessment_id: row.assessment_id,
        question_id: row.question_id,
        answer: row.answer,
        score: row.score,
        is_correct: row.is_correct,
    }
}

fn row_to_credit(row: CreditRow) -> Credit {
    Credit {
        id: row.id,
        student_id: row.student_id,
        language_id: row.language_id,
        grade: row.grade,
        semester: row.semester,
        score: row.score,
        grade_letter: row.grade_letter,
        credit_points: row.credit_points,
        status: row.status,
        certificate_url: row.certificate_url,
        verification_code: row.verification_code,
        issued_at: row.issued_at,
    }
}
