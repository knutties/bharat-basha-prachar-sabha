use chrono::Utc;
use uuid::Uuid;

use super::errors::PaymentError;
use super::models::{
    CreateSubscriptionInput, Plan, Subscription,
};
use super::repository::PaymentRepository;

/// Core payment service containing all business logic.
pub struct PaymentServiceImpl<R: PaymentRepository> {
    repo: R,
}

impl<R: PaymentRepository> PaymentServiceImpl<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// List all active plans.
    pub fn list_plans(&self) -> Result<Vec<Plan>, PaymentError> {
        self.repo.list_active_plans()
    }

    /// Create a new subscription for a user.
    pub fn create_subscription(
        &self,
        input: CreateSubscriptionInput,
    ) -> Result<Subscription, PaymentError> {
        // Verify plan exists
        let _plan = self
            .repo
            .find_plan_by_id(input.plan_id)?
            .ok_or_else(|| {
                PaymentError::PlanNotFound(input.plan_id.to_string())
            })?;

        // Check if user already has an active subscription
        if self
            .repo
            .find_active_subscription_by_user(input.user_id)?
            .is_some()
        {
            return Err(PaymentError::AlreadySubscribed(
                input.user_id.to_string(),
            ));
        }

        let now = Utc::now();
        let subscription = Subscription {
            id: Uuid::new_v4(),
            user_id: input.user_id,
            plan_id: input.plan_id,
            status: "pending".to_string(),
            razorpay_subscription_id: None,
            current_period_start: Some(now),
            current_period_end: None,
            created_at: now,
        };

        // TODO: Create Razorpay subscription and update razorpay_subscription_id

        self.repo.create_subscription(&subscription)
    }

    /// Get a subscription by ID.
    pub fn get_subscription(
        &self,
        id: Uuid,
    ) -> Result<Subscription, PaymentError> {
        self.repo
            .find_subscription_by_id(id)?
            .ok_or_else(|| {
                PaymentError::SubscriptionNotFound(id.to_string())
            })
    }

    /// Cancel a subscription.
    pub fn cancel_subscription(
        &self,
        id: Uuid,
    ) -> Result<Subscription, PaymentError> {
        let mut subscription = self
            .repo
            .find_subscription_by_id(id)?
            .ok_or_else(|| {
                PaymentError::SubscriptionNotFound(id.to_string())
            })?;

        subscription.status = "cancelled".to_string();

        // TODO: Cancel Razorpay subscription

        self.repo.update_subscription(&subscription)
    }

    /// Handle incoming Razorpay webhook events.
    pub fn handle_razorpay_webhook(
        &self,
        event: &str,
        _payload: &serde_json::Value,
    ) -> Result<(), PaymentError> {
        // TODO: Validate webhook signature

        match event {
            "payment.captured" => {
                // TODO: Update payment status, activate subscription
                tracing::info!("Payment captured webhook received");
                Ok(())
            }
            "subscription.activated" => {
                tracing::info!("Subscription activated webhook received");
                Ok(())
            }
            "subscription.cancelled" => {
                tracing::info!("Subscription cancelled webhook received");
                Ok(())
            }
            "payment.failed" => {
                tracing::warn!("Payment failed webhook received");
                Ok(())
            }
            _ => {
                tracing::debug!("Unhandled webhook event: {}", event);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repository::MockPaymentRepository;

    #[test]
    fn test_create_subscription_plan_not_found() {
        let mut mock_repo = MockPaymentRepository::new();
        mock_repo
            .expect_find_plan_by_id()
            .returning(|_| Ok(None));

        let service = PaymentServiceImpl::new(mock_repo);
        let input = CreateSubscriptionInput {
            user_id: Uuid::new_v4(),
            plan_id: Uuid::new_v4(),
        };

        let result = service.create_subscription(input);
        assert!(matches!(result, Err(PaymentError::PlanNotFound(_))));
    }

    #[test]
    fn test_create_subscription_success() {
        let mut mock_repo = MockPaymentRepository::new();

        mock_repo.expect_find_plan_by_id().returning(|id| {
            Ok(Some(Plan {
                id,
                name: "Basic".to_string(),
                price_paise: 29900,
                currency: "INR".to_string(),
                interval: "monthly".to_string(),
                features: None,
                active: true,
            }))
        });

        mock_repo
            .expect_find_active_subscription_by_user()
            .returning(|_| Ok(None));

        mock_repo
            .expect_create_subscription()
            .returning(|s| Ok(s.clone()));

        let service = PaymentServiceImpl::new(mock_repo);
        let input = CreateSubscriptionInput {
            user_id: Uuid::new_v4(),
            plan_id: Uuid::new_v4(),
        };

        let result = service.create_subscription(input);
        assert!(result.is_ok());
        let sub = result.unwrap();
        assert_eq!(sub.status, "pending");
    }
}
