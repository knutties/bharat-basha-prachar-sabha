use uuid::Uuid;

use super::errors::PaymentError;
use super::models::{Payment, Plan, Subscription};

/// Repository trait for payment persistence.
/// Implemented by the database layer; consumed by domain services.
#[cfg_attr(test, mockall::automock)]
pub trait PaymentRepository: Send + Sync {
    /// List all active plans.
    fn list_active_plans(&self) -> Result<Vec<Plan>, PaymentError>;

    /// Find a plan by ID.
    fn find_plan_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Plan>, PaymentError>;

    /// Create a subscription.
    fn create_subscription(
        &self,
        subscription: &Subscription,
    ) -> Result<Subscription, PaymentError>;

    /// Find a subscription by ID.
    fn find_subscription_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Subscription>, PaymentError>;

    /// Find active subscription for a user.
    fn find_active_subscription_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Option<Subscription>, PaymentError>;

    /// Update a subscription.
    fn update_subscription(
        &self,
        subscription: &Subscription,
    ) -> Result<Subscription, PaymentError>;

    /// Create a payment record.
    fn create_payment(
        &self,
        payment: &Payment,
    ) -> Result<Payment, PaymentError>;

    /// Update a payment record.
    fn update_payment(
        &self,
        payment: &Payment,
    ) -> Result<Payment, PaymentError>;

    /// Find a payment by Razorpay payment ID.
    fn find_payment_by_razorpay_id(
        &self,
        razorpay_payment_id: &str,
    ) -> Result<Option<Payment>, PaymentError>;
}
