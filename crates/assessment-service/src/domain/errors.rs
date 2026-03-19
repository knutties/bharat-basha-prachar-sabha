/// Domain-specific errors for the assessment service.
#[derive(Debug, thiserror::Error)]
pub enum AssessmentError {
    #[error("Assessment not found: {0}")]
    AssessmentNotFound(String),

    #[error("Assessment already submitted: {0}")]
    AlreadySubmitted(String),

    #[error("Time limit exceeded for assessment: {0}")]
    TimeLimitExceeded(String),

    #[error("Credit not found: {0}")]
    CreditNotFound(String),

    #[error("Invalid verification code: {0}")]
    InvalidVerificationCode(String),

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),
}
