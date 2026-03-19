/// Domain-specific errors for the live class service.
#[derive(Debug, thiserror::Error)]
pub enum LiveClassError {
    #[error("Class not found: {0}")]
    ClassNotFound(String),

    #[error("Class is full: {0}")]
    ClassFull(String),

    #[error("Already enrolled in class: {0}")]
    AlreadyEnrolled(String),

    #[error("Teacher not found: {0}")]
    TeacherNotFound(String),

    #[error("Schedule conflict: {0}")]
    ScheduleConflict(String),

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),
}
