/// Domain-specific errors for the content service.
#[derive(Debug, thiserror::Error)]
pub enum ContentError {
    #[error("Content not found: {0}")]
    ContentNotFound(String),

    #[error("Media not found: {0}")]
    MediaNotFound(String),

    #[error("Invalid content type: {0}")]
    InvalidContentType(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),
}
