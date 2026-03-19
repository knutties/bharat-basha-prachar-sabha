use diesel::prelude::*;
use uuid::Uuid;

use super::connection::DbPool;
use super::models::*;
use super::schema::{payments, plans, subscriptions};
use crate::domain::errors::PaymentError;
use crate::domain::models::{Payment, Plan, Subscription};
use crate::domain::repository::PaymentRepository;

/// PostgreSQL implementation of the PaymentRepository trait.
pub struct PgPaymentRepository {
    pool: DbPool,
}

impl PgPaymentRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn get_conn(
        &self,
    ) -> Result<
        diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<PgConnection>,
        >,
        PaymentError,
    > {
        self.pool
            .get()
            .map_err(|e| PaymentError::RepositoryError(e.to_string()))
    }
}

impl PaymentRepository for PgPaymentRepository {
    fn list_active_plans(&self) -> Result<Vec<Plan>, PaymentError> {
        let mut conn = self.get_conn()?;

        let rows = plans::table
            .filter(plans::active.eq(true))
            .select(PlanRow::as_select())
            .load::<PlanRow>(&mut conn)
            .map_err(|e| PaymentError::RepositoryError(e.to_string()))?;

        Ok(rows.into_iter().map(row_to_plan).collect())
    }

    fn find_plan_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Plan>, PaymentError> {
        let mut conn = self.get_conn()?;

        let row = plans::table
            .filter(plans::id.eq(id))
            .select(PlanRow::as_select())
            .first::<PlanRow>(&mut conn)
            .optional()
            .map_err(|e| PaymentError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_plan))
    }

    fn create_subscription(
        &self,
        subscription: &Subscription,
    ) -> Result<Subscription, PaymentError> {
        let mut conn = self.get_conn()?;

        let new_row = NewSubscriptionRow {
            id: subscription.id,
            user_id: subscription.user_id,
            plan_id: subscription.plan_id,
            status: &subscription.status,
            razorpay_subscription_id: subscription
                .razorpay_subscription_id
                .as_deref(),
            current_period_start: subscription.current_period_start,
            current_period_end: subscription.current_period_end,
            created_at: subscription.created_at,
        };

        let row = diesel::insert_into(subscriptions::table)
            .values(&new_row)
            .returning(SubscriptionRow::as_returning())
            .get_result::<SubscriptionRow>(&mut conn)
            .map_err(|e| PaymentError::RepositoryError(e.to_string()))?;

        Ok(row_to_subscription(row))
    }

    fn find_subscription_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Subscription>, PaymentError> {
        let mut conn = self.get_conn()?;

        let row = subscriptions::table
            .filter(subscriptions::id.eq(id))
            .select(SubscriptionRow::as_select())
            .first::<SubscriptionRow>(&mut conn)
            .optional()
            .map_err(|e| PaymentError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_subscription))
    }

    fn find_active_subscription_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Option<Subscription>, PaymentError> {
        let mut conn = self.get_conn()?;

        let row = subscriptions::table
            .filter(
                subscriptions::user_id
                    .eq(user_id)
                    .and(subscriptions::status.eq("active")),
            )
            .select(SubscriptionRow::as_select())
            .first::<SubscriptionRow>(&mut conn)
            .optional()
            .map_err(|e| PaymentError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_subscription))
    }

    fn update_subscription(
        &self,
        subscription: &Subscription,
    ) -> Result<Subscription, PaymentError> {
        let mut conn = self.get_conn()?;

        diesel::update(
            subscriptions::table
                .filter(subscriptions::id.eq(subscription.id)),
        )
        .set((
            subscriptions::status.eq(&subscription.status),
            subscriptions::razorpay_subscription_id
                .eq(&subscription.razorpay_subscription_id),
            subscriptions::current_period_start
                .eq(subscription.current_period_start),
            subscriptions::current_period_end
                .eq(subscription.current_period_end),
        ))
        .execute(&mut conn)
        .map_err(|e| PaymentError::RepositoryError(e.to_string()))?;

        self.find_subscription_by_id(subscription.id)?
            .ok_or_else(|| {
                PaymentError::SubscriptionNotFound(
                    subscription.id.to_string(),
                )
            })
    }

    fn create_payment(
        &self,
        payment: &Payment,
    ) -> Result<Payment, PaymentError> {
        let mut conn = self.get_conn()?;

        let new_row = NewPaymentRow {
            id: payment.id,
            subscription_id: payment.subscription_id,
            user_id: payment.user_id,
            amount_paise: payment.amount_paise,
            currency: &payment.currency,
            status: &payment.status,
            razorpay_payment_id: payment.razorpay_payment_id.as_deref(),
            razorpay_order_id: payment.razorpay_order_id.as_deref(),
            paid_at: payment.paid_at,
        };

        let row = diesel::insert_into(payments::table)
            .values(&new_row)
            .returning(PaymentRow::as_returning())
            .get_result::<PaymentRow>(&mut conn)
            .map_err(|e| PaymentError::RepositoryError(e.to_string()))?;

        Ok(row_to_payment(row))
    }

    fn update_payment(
        &self,
        payment: &Payment,
    ) -> Result<Payment, PaymentError> {
        let mut conn = self.get_conn()?;

        diesel::update(
            payments::table.filter(payments::id.eq(payment.id)),
        )
        .set((
            payments::status.eq(&payment.status),
            payments::razorpay_payment_id
                .eq(&payment.razorpay_payment_id),
            payments::paid_at.eq(payment.paid_at),
        ))
        .execute(&mut conn)
        .map_err(|e| PaymentError::RepositoryError(e.to_string()))?;

        // Re-fetch
        let mut conn = self.get_conn()?;
        let row = payments::table
            .filter(payments::id.eq(payment.id))
            .select(PaymentRow::as_select())
            .first::<PaymentRow>(&mut conn)
            .map_err(|e| PaymentError::RepositoryError(e.to_string()))?;

        Ok(row_to_payment(row))
    }

    fn find_payment_by_razorpay_id(
        &self,
        razorpay_payment_id: &str,
    ) -> Result<Option<Payment>, PaymentError> {
        let mut conn = self.get_conn()?;

        let row = payments::table
            .filter(
                payments::razorpay_payment_id.eq(razorpay_payment_id),
            )
            .select(PaymentRow::as_select())
            .first::<PaymentRow>(&mut conn)
            .optional()
            .map_err(|e| PaymentError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_payment))
    }
}

fn row_to_plan(row: PlanRow) -> Plan {
    Plan {
        id: row.id,
        name: row.name,
        price_paise: row.price_paise,
        currency: row.currency,
        interval: row.interval,
        features: row.features,
        active: row.active,
    }
}

fn row_to_subscription(row: SubscriptionRow) -> Subscription {
    Subscription {
        id: row.id,
        user_id: row.user_id,
        plan_id: row.plan_id,
        status: row.status,
        razorpay_subscription_id: row.razorpay_subscription_id,
        current_period_start: row.current_period_start,
        current_period_end: row.current_period_end,
        created_at: row.created_at,
    }
}

fn row_to_payment(row: PaymentRow) -> Payment {
    Payment {
        id: row.id,
        subscription_id: row.subscription_id,
        user_id: row.user_id,
        amount_paise: row.amount_paise,
        currency: row.currency,
        status: row.status,
        razorpay_payment_id: row.razorpay_payment_id,
        razorpay_order_id: row.razorpay_order_id,
        paid_at: row.paid_at,
    }
}
