use diesel::prelude::*;
use shared::types::{Grade, ProficiencyLevel, SkillType};
use uuid::Uuid;

use super::connection::DbPool;
use super::models::*;
use super::schema::{curricula, enrollments, languages, lessons, modules, progress};
use crate::domain::errors::CurriculumError;
use crate::domain::models::{
    self as domain, Curriculum, Enrollment, Language, Lesson, Module, Progress,
};
use crate::domain::repository::{CurriculumRepository, EnrollmentRepository};

// ─── PostgreSQL Curriculum Repository ───────────────────────────

/// PostgreSQL implementation of the CurriculumRepository trait.
pub struct PgCurriculumRepository {
    pool: DbPool,
}

impl PgCurriculumRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn get_conn(
        &self,
    ) -> Result<
        diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<PgConnection>,
        >,
        CurriculumError,
    > {
        self.pool
            .get()
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))
    }
}

impl CurriculumRepository for PgCurriculumRepository {
    fn list_languages(&self) -> Result<Vec<Language>, CurriculumError> {
        let mut conn = self.get_conn()?;

        let rows = languages::table
            .select(LanguageRow::as_select())
            .load::<LanguageRow>(&mut conn)
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))?;

        Ok(rows.into_iter().map(row_to_language).collect())
    }

    fn find_language_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Language>, CurriculumError> {
        let mut conn = self.get_conn()?;

        let row = languages::table
            .filter(languages::id.eq(id))
            .select(LanguageRow::as_select())
            .first::<LanguageRow>(&mut conn)
            .optional()
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_language))
    }

    fn find_curriculum(
        &self,
        language_id: Uuid,
        grade: Option<Grade>,
    ) -> Result<Option<Curriculum>, CurriculumError> {
        let mut conn = self.get_conn()?;

        let mut query = curricula::table
            .filter(curricula::language_id.eq(language_id))
            .into_boxed();

        if let Some(g) = grade {
            query = query.filter(curricula::grade.eq(g.value() as i16));
        }

        let row = query
            .select(CurriculumRow::as_select())
            .first::<CurriculumRow>(&mut conn)
            .optional()
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_curriculum))
    }

    fn list_modules(
        &self,
        curriculum_id: Uuid,
        skill_type: Option<SkillType>,
    ) -> Result<Vec<Module>, CurriculumError> {
        let mut conn = self.get_conn()?;

        let mut query = modules::table
            .filter(modules::curriculum_id.eq(curriculum_id))
            .into_boxed();

        if let Some(st) = skill_type {
            query = query
                .filter(modules::skill_type.eq(format!("{:?}", st)));
        }

        let rows = query
            .order(modules::order_index.asc())
            .select(ModuleRow::as_select())
            .load::<ModuleRow>(&mut conn)
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))?;

        Ok(rows.into_iter().map(row_to_module).collect())
    }

    fn find_module(
        &self,
        module_id: Uuid,
    ) -> Result<Option<Module>, CurriculumError> {
        let mut conn = self.get_conn()?;

        let row = modules::table
            .filter(modules::id.eq(module_id))
            .select(ModuleRow::as_select())
            .first::<ModuleRow>(&mut conn)
            .optional()
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_module))
    }

    fn find_lesson(
        &self,
        lesson_id: Uuid,
    ) -> Result<Option<Lesson>, CurriculumError> {
        let mut conn = self.get_conn()?;

        let row = lessons::table
            .filter(lessons::id.eq(lesson_id))
            .select(LessonRow::as_select())
            .first::<LessonRow>(&mut conn)
            .optional()
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_lesson))
    }

    fn list_lessons_for_module(
        &self,
        module_id: Uuid,
    ) -> Result<Vec<Lesson>, CurriculumError> {
        let mut conn = self.get_conn()?;

        let rows = lessons::table
            .filter(lessons::module_id.eq(module_id))
            .order(lessons::order_index.asc())
            .select(LessonRow::as_select())
            .load::<LessonRow>(&mut conn)
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))?;

        Ok(rows.into_iter().map(row_to_lesson).collect())
    }
}

// ─── PostgreSQL Enrollment Repository ───────────────────────────

/// PostgreSQL implementation of the EnrollmentRepository trait.
pub struct PgEnrollmentRepository {
    pool: DbPool,
}

impl PgEnrollmentRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn get_conn(
        &self,
    ) -> Result<
        diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<PgConnection>,
        >,
        CurriculumError,
    > {
        self.pool
            .get()
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))
    }
}

impl EnrollmentRepository for PgEnrollmentRepository {
    fn create_enrollment(
        &self,
        enrollment: &Enrollment,
    ) -> Result<Enrollment, CurriculumError> {
        let mut conn = self.get_conn()?;

        let new_row = NewEnrollmentRow {
            id: enrollment.id,
            student_id: enrollment.student_id,
            language_id: enrollment.language_id,
            grade: enrollment.grade.value() as i16,
            proficiency: &format!("{:?}", enrollment.proficiency),
            enrolled_at: enrollment.enrolled_at,
        };

        let row = diesel::insert_into(enrollments::table)
            .values(&new_row)
            .returning(EnrollmentRow::as_returning())
            .get_result::<EnrollmentRow>(&mut conn)
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))?;

        Ok(row_to_enrollment(row))
    }

    fn find_enrollment(
        &self,
        enrollment_id: Uuid,
    ) -> Result<Option<Enrollment>, CurriculumError> {
        let mut conn = self.get_conn()?;

        let row = enrollments::table
            .filter(enrollments::id.eq(enrollment_id))
            .select(EnrollmentRow::as_select())
            .first::<EnrollmentRow>(&mut conn)
            .optional()
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_enrollment))
    }

    fn find_enrollments_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<Enrollment>, CurriculumError> {
        let mut conn = self.get_conn()?;

        let rows = enrollments::table
            .filter(enrollments::student_id.eq(student_id))
            .select(EnrollmentRow::as_select())
            .load::<EnrollmentRow>(&mut conn)
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))?;

        Ok(rows.into_iter().map(row_to_enrollment).collect())
    }

    fn find_enrollment_by_student_and_language(
        &self,
        student_id: Uuid,
        language_id: Uuid,
    ) -> Result<Option<Enrollment>, CurriculumError> {
        let mut conn = self.get_conn()?;

        let row = enrollments::table
            .filter(
                enrollments::student_id
                    .eq(student_id)
                    .and(enrollments::language_id.eq(language_id)),
            )
            .select(EnrollmentRow::as_select())
            .first::<EnrollmentRow>(&mut conn)
            .optional()
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_enrollment))
    }

    fn record_progress(
        &self,
        prog: &Progress,
    ) -> Result<Progress, CurriculumError> {
        let mut conn = self.get_conn()?;

        let new_row = NewProgressRow {
            id: prog.id,
            student_id: prog.student_id,
            lesson_id: prog.lesson_id,
            completed_at: prog.completed_at,
            score: prog.score,
            time_spent_seconds: prog.time_spent_seconds,
        };

        let row = diesel::insert_into(progress::table)
            .values(&new_row)
            .returning(ProgressRow::as_returning())
            .get_result::<ProgressRow>(&mut conn)
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))?;

        Ok(row_to_progress(row))
    }

    fn get_module_progress(
        &self,
        student_id: Uuid,
        module_id: Uuid,
    ) -> Result<Vec<Progress>, CurriculumError> {
        let mut conn = self.get_conn()?;

        // Get all lesson IDs for the module, then find progress for those lessons.
        let lesson_ids: Vec<Uuid> = lessons::table
            .filter(lessons::module_id.eq(module_id))
            .select(lessons::id)
            .load::<Uuid>(&mut conn)
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))?;

        let rows = progress::table
            .filter(
                progress::student_id
                    .eq(student_id)
                    .and(progress::lesson_id.eq_any(&lesson_ids)),
            )
            .select(ProgressRow::as_select())
            .load::<ProgressRow>(&mut conn)
            .map_err(|e| CurriculumError::RepositoryError(e.to_string()))?;

        Ok(rows.into_iter().map(row_to_progress).collect())
    }
}

// ─── Row Conversion Helpers ─────────────────────────────────────

fn row_to_language(row: LanguageRow) -> Language {
    Language {
        id: row.id,
        name: row.name,
        script: row.script,
        iso_code: row.iso_code,
        family: row.family,
    }
}

fn row_to_curriculum(row: CurriculumRow) -> Curriculum {
    Curriculum {
        id: row.id,
        language_id: row.language_id,
        grade: Grade::new(row.grade as u8)
            .unwrap_or_else(|| Grade::new(1).unwrap()),
        title: row.title,
        description: row.description,
    }
}

fn row_to_module(row: ModuleRow) -> Module {
    let skill_type = match row.skill_type.as_str() {
        "Reading" => SkillType::Reading,
        "Writing" => SkillType::Writing,
        "Listening" => SkillType::Listening,
        "Speaking" => SkillType::Speaking,
        _ => SkillType::Reading,
    };

    Module {
        id: row.id,
        curriculum_id: row.curriculum_id,
        title: row.title,
        description: row.description,
        skill_type,
        order_index: row.order_index,
        estimated_minutes: row.estimated_minutes,
    }
}

fn row_to_lesson(row: LessonRow) -> Lesson {
    let lesson_type = domain::LessonType::from_str(&row.lesson_type)
        .unwrap_or(domain::LessonType::Text);

    Lesson {
        id: row.id,
        module_id: row.module_id,
        title: row.title,
        description: row.description,
        lesson_type,
        content: row.content,
        media_urls: row.media_urls,
        duration_minutes: row.duration_minutes,
        order_index: row.order_index,
    }
}

fn row_to_enrollment(row: EnrollmentRow) -> Enrollment {
    let proficiency = match row.proficiency.as_str() {
        "Beginner" => ProficiencyLevel::Beginner,
        "Intermediate" => ProficiencyLevel::Intermediate,
        "Advanced" => ProficiencyLevel::Advanced,
        _ => ProficiencyLevel::Beginner,
    };

    Enrollment {
        id: row.id,
        student_id: row.student_id,
        language_id: row.language_id,
        grade: Grade::new(row.grade as u8)
            .unwrap_or_else(|| Grade::new(1).unwrap()),
        proficiency,
        enrolled_at: row.enrolled_at,
    }
}

fn row_to_progress(row: ProgressRow) -> Progress {
    Progress {
        id: row.id,
        student_id: row.student_id,
        lesson_id: row.lesson_id,
        completed_at: row.completed_at,
        score: row.score,
        time_spent_seconds: row.time_spent_seconds,
    }
}
