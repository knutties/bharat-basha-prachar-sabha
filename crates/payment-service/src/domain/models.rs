use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

/// A subscription plan.
#[derive(Debug, Clone, Serialize)]
pub struct Plan {
    pub id: Uuid,
    pub name: String,
    pub price_paise: i64,
    pub currency: String,
    pub interval: String,
    pub features: Option<String>,
    pub active: bool,
}

/// A user subscription.
#[derive(Debug, Clone, Serialize)]
pub struct Subscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plan_id: Uuid,
    pub status: String,
    pub razorpay_subscription_id: Option<String>,
    pub current_period_start: Option<DateTime<Utc>>,
    pub current_period_end: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// A payment record.
#[derive(Debug, Clone, Serialize)]
pub struct Payment {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub user_id: Uuid,
    pub amount_paise: i64,
    pub currency: String,
    pub status: String,
    pub razorpay_payment_id: Option<String>,
    pub razorpay_order_id: Option<String>,
    pub paid_at: Option<DateTime<Utc>>,
}

/// Input for creating a new subscription.
#[derive(Debug)]
pub struct CreateSubscriptionInput {
    pub user_id: Uuid,
    pub plan_id: Uuid,
}
