/// Domain-specific errors for the payment service.
#[derive(Debug, thiserror::Error)]
pub enum PaymentError {
    #[error("Plan not found: {0}")]
    PlanNotFound(String),

    #[error("Subscription not found: {0}")]
    SubscriptionNotFound(String),

    #[error("Payment failed: {0}")]
    PaymentFailed(String),

    #[error("User already has an active subscription: {0}")]
    AlreadySubscribed(String),

    #[error("Webhook validation failed: {0}")]
    WebhookValidationFailed(String),

    #[error("Validation failed: {0}")]
    ValidationError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),
}
