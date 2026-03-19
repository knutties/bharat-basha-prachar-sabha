use shared::types::{Grade, SkillType};
use uuid::Uuid;

use super::errors::CurriculumError;
use super::models::{
    Curriculum, Enrollment, Language, Lesson, Module, Progress,
};

/// Repository trait for curriculum data (languages, curricula, modules, lessons).
/// Implemented by the database layer; consumed by domain services.
#[cfg_attr(test, mockall::automock)]
pub trait CurriculumRepository: Send + Sync {
    /// List all available languages.
    fn list_languages(&self) -> Result<Vec<Language>, CurriculumError>;

    /// Find a language by its ID.
    fn find_language_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Language>, CurriculumError>;

    /// Find a curriculum for a given language and optional grade filter.
    fn find_curriculum(
        &self,
        language_id: Uuid,
        grade: Option<Grade>,
    ) -> Result<Option<Curriculum>, CurriculumError>;

    /// List modules for a curriculum, optionally filtered by skill type.
    fn list_modules(
        &self,
        curriculum_id: Uuid,
        skill_type: Option<SkillType>,
    ) -> Result<Vec<Module>, CurriculumError>;

    /// Find a module by its ID.
    fn find_module(
        &self,
        module_id: Uuid,
    ) -> Result<Option<Module>, CurriculumError>;

    /// Find a lesson by its ID.
    fn find_lesson(
        &self,
        lesson_id: Uuid,
    ) -> Result<Option<Lesson>, CurriculumError>;

    /// List all lessons for a module, ordered by order_index.
    fn list_lessons_for_module(
        &self,
        module_id: Uuid,
    ) -> Result<Vec<Lesson>, CurriculumError>;
}

/// Repository trait for enrollment and progress data.
/// Implemented by the database layer; consumed by domain services.
#[cfg_attr(test, mockall::automock)]
pub trait EnrollmentRepository: Send + Sync {
    /// Create a new enrollment record.
    fn create_enrollment(
        &self,
        enrollment: &Enrollment,
    ) -> Result<Enrollment, CurriculumError>;

    /// Find an enrollment by its ID.
    fn find_enrollment(
        &self,
        enrollment_id: Uuid,
    ) -> Result<Option<Enrollment>, CurriculumError>;

    /// Find all enrollments for a student.
    fn find_enrollments_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<Enrollment>, CurriculumError>;

    /// Check if a student is already enrolled in a language.
    fn find_enrollment_by_student_and_language(
        &self,
        student_id: Uuid,
        language_id: Uuid,
    ) -> Result<Option<Enrollment>, CurriculumError>;

    /// Record lesson progress for a student.
    fn record_progress(
        &self,
        progress: &Progress,
    ) -> Result<Progress, CurriculumError>;

    /// Get all progress records for a student within a specific module.
    fn get_module_progress(
        &self,
        student_id: Uuid,
        module_id: Uuid,
    ) -> Result<Vec<Progress>, CurriculumError>;
}
