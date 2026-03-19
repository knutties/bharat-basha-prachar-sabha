/// Domain-specific errors for the auth service.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Email already registered: {0}")]
    EmailAlreadyExists(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Password hashing failed: {0}")]
    PasswordHashError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),
}
