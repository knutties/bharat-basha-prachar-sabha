/// Domain-specific errors for the curriculum service.
#[derive(Debug, thiserror::Error)]
pub enum CurriculumError {
    #[error("Language not found: {0}")]
    LanguageNotFound(String),

    #[error("Curriculum not found: {0}")]
    CurriculumNotFound(String),

    #[error("Module not found: {0}")]
    ModuleNotFound(String),

    #[error("Lesson not found: {0}")]
    LessonNotFound(String),

    #[error("Enrollment not found: {0}")]
    EnrollmentNotFound(String),

    #[error("Student is already enrolled in this language: {0}")]
    AlreadyEnrolled(String),

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),
}
