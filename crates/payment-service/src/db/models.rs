use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use super::schema::{payments, plans, subscriptions};

/// Diesel model for reading plans.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = plans)]
pub struct PlanRow {
    pub id: Uuid,
    pub name: String,
    pub price_paise: i64,
    pub currency: String,
    pub interval: String,
    pub features: Option<String>,
    pub active: bool,
}

/// Diesel model for inserting plans.
#[derive(Insertable)]
#[diesel(table_name = plans)]
pub struct NewPlanRow<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub price_paise: i64,
    pub currency: &'a str,
    pub interval: &'a str,
    pub features: Option<&'a str>,
    pub active: bool,
}

/// Diesel model for reading subscriptions.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = subscriptions)]
pub struct SubscriptionRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plan_id: Uuid,
    pub status: String,
    pub razorpay_subscription_id: Option<String>,
    pub current_period_start: Option<DateTime<Utc>>,
    pub current_period_end: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Diesel model for inserting subscriptions.
#[derive(Insertable)]
#[diesel(table_name = subscriptions)]
pub struct NewSubscriptionRow<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plan_id: Uuid,
    pub status: &'a str,
    pub razorpay_subscription_id: Option<&'a str>,
    pub current_period_start: Option<DateTime<Utc>>,
    pub current_period_end: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Diesel model for reading payments.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = payments)]
pub struct PaymentRow {
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

/// Diesel model for inserting payments.
#[derive(Insertable)]
#[diesel(table_name = payments)]
pub struct NewPaymentRow<'a> {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub user_id: Uuid,
    pub amount_paise: i64,
    pub currency: &'a str,
    pub status: &'a str,
    pub razorpay_payment_id: Option<&'a str>,
    pub razorpay_order_id: Option<&'a str>,
    pub paid_at: Option<DateTime<Utc>>,
}
