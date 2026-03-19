use serde::Serialize;

/// Common error types shared across all services.
/// Each service extends this with domain-specific variants.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Entity not found: {entity} with id {id}")]
    NotFound { entity: String, id: String },

    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Authentication required")]
    Unauthorized,

    #[error("Insufficient permissions")]
    Forbidden,

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Serializable error response body for API responses.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorBody,
}

/// Error body containing code and human-readable message.
#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
}

impl AppError {
    /// HTTP status code for this error.
    pub fn status_code(&self) -> u16 {
        match self {
            AppError::NotFound { .. } => 404,
            AppError::Validation(_) => 400,
            AppError::Unauthorized => 401,
            AppError::Forbidden => 403,
            AppError::Conflict(_) => 409,
            AppError::Internal(_) => 500,
        }
    }

    /// Machine-readable error code.
    pub fn error_code(&self) -> &str {
        match self {
            AppError::NotFound { .. } => "NOT_FOUND",
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::Unauthorized => "UNAUTHORIZED",
            AppError::Forbidden => "FORBIDDEN",
            AppError::Conflict(_) => "CONFLICT",
            AppError::Internal(_) => "INTERNAL_ERROR",
        }
    }

    /// Convert to API error response.
    pub fn to_response(&self) -> ErrorResponse {
        ErrorResponse {
            error: ErrorBody {
                code: self.error_code().to_string(),
                message: self.to_string(),
            },
        }
    }
}
