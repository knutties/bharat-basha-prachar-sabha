/// Domain-specific errors for the notification service.
#[derive(Debug, thiserror::Error)]
pub enum NotificationError {
    #[error("Notification not found: {0}")]
    NotificationNotFound(String),

    #[error("User not found: {0}")]
    UserNotFound(String),

    #[error("Invalid channel: {0}")]
    InvalidChannel(String),

    #[error("Delivery failed: {0}")]
    DeliveryFailed(String),

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),
}
