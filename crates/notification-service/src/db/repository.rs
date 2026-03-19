use diesel::prelude::*;
use uuid::Uuid;

use super::connection::DbPool;
use super::models::*;
use super::schema::{notification_preferences, notifications};
use crate::domain::errors::NotificationError;
use crate::domain::models::{Notification, NotificationPreference};
use crate::domain::repository::NotificationRepository;

/// PostgreSQL implementation of the NotificationRepository trait.
pub struct PgNotificationRepository {
    pool: DbPool,
}

impl PgNotificationRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn get_conn(
        &self,
    ) -> Result<
        diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<PgConnection>,
        >,
        NotificationError,
    > {
        self.pool
            .get()
            .map_err(|e| NotificationError::RepositoryError(e.to_string()))
    }
}

impl NotificationRepository for PgNotificationRepository {
    fn create_notification(
        &self,
        notification: &Notification,
    ) -> Result<Notification, NotificationError> {
        let mut conn = self.get_conn()?;

        let new_row = NewNotificationRow {
            id: notification.id,
            user_id: notification.user_id,
            notification_type: &notification.notification_type,
            channel: &notification.channel,
            title: &notification.title,
            body: &notification.body,
            status: &notification.status,
            sent_at: notification.sent_at,
            read_at: notification.read_at,
        };

        let row = diesel::insert_into(notifications::table)
            .values(&new_row)
            .returning(NotificationRow::as_returning())
            .get_result::<NotificationRow>(&mut conn)
            .map_err(|e| {
                NotificationError::RepositoryError(e.to_string())
            })?;

        Ok(row_to_notification(row))
    }

    fn find_notification_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Notification>, NotificationError> {
        let mut conn = self.get_conn()?;

        let row = notifications::table
            .filter(notifications::id.eq(id))
            .select(NotificationRow::as_select())
            .first::<NotificationRow>(&mut conn)
            .optional()
            .map_err(|e| {
                NotificationError::RepositoryError(e.to_string())
            })?;

        Ok(row.map(row_to_notification))
    }

    fn find_notifications_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Notification>, NotificationError> {
        let mut conn = self.get_conn()?;

        let rows = notifications::table
            .filter(notifications::user_id.eq(user_id))
            .order(notifications::sent_at.desc())
            .select(NotificationRow::as_select())
            .load::<NotificationRow>(&mut conn)
            .map_err(|e| {
                NotificationError::RepositoryError(e.to_string())
            })?;

        Ok(rows.into_iter().map(row_to_notification).collect())
    }

    fn update_notification(
        &self,
        notification: &Notification,
    ) -> Result<Notification, NotificationError> {
        let mut conn = self.get_conn()?;

        diesel::update(
            notifications::table
                .filter(notifications::id.eq(notification.id)),
        )
        .set((
            notifications::status.eq(&notification.status),
            notifications::read_at.eq(notification.read_at),
        ))
        .execute(&mut conn)
        .map_err(|e| {
            NotificationError::RepositoryError(e.to_string())
        })?;

        self.find_notification_by_id(notification.id)?
            .ok_or_else(|| {
                NotificationError::NotificationNotFound(
                    notification.id.to_string(),
                )
            })
    }

    fn find_preferences_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Option<NotificationPreference>, NotificationError> {
        let mut conn = self.get_conn()?;

        let row = notification_preferences::table
            .filter(notification_preferences::user_id.eq(user_id))
            .select(NotificationPreferenceRow::as_select())
            .first::<NotificationPreferenceRow>(&mut conn)
            .optional()
            .map_err(|e| {
                NotificationError::RepositoryError(e.to_string())
            })?;

        Ok(row.map(row_to_preference))
    }

    fn upsert_preferences(
        &self,
        preferences: &NotificationPreference,
    ) -> Result<NotificationPreference, NotificationError> {
        let mut conn = self.get_conn()?;

        let new_row = NewNotificationPreferenceRow {
            user_id: preferences.user_id,
            email_enabled: preferences.email_enabled,
            sms_enabled: preferences.sms_enabled,
            push_enabled: preferences.push_enabled,
            digest_frequency: preferences.digest_frequency.as_deref(),
        };

        let row = diesel::insert_into(notification_preferences::table)
            .values(&new_row)
            .on_conflict(notification_preferences::user_id)
            .do_update()
            .set((
                notification_preferences::email_enabled
                    .eq(preferences.email_enabled),
                notification_preferences::sms_enabled
                    .eq(preferences.sms_enabled),
                notification_preferences::push_enabled
                    .eq(preferences.push_enabled),
                notification_preferences::digest_frequency
                    .eq(&preferences.digest_frequency),
            ))
            .returning(NotificationPreferenceRow::as_returning())
            .get_result::<NotificationPreferenceRow>(&mut conn)
            .map_err(|e| {
                NotificationError::RepositoryError(e.to_string())
            })?;

        Ok(row_to_preference(row))
    }
}

fn row_to_notification(row: NotificationRow) -> Notification {
    Notification {
        id: row.id,
        user_id: row.user_id,
        notification_type: row.notification_type,
        channel: row.channel,
        title: row.title,
        body: row.body,
        status: row.status,
        sent_at: row.sent_at,
        read_at: row.read_at,
    }
}

fn row_to_preference(row: NotificationPreferenceRow) -> NotificationPreference {
    NotificationPreference {
        user_id: row.user_id,
        email_enabled: row.email_enabled,
        sms_enabled: row.sms_enabled,
        push_enabled: row.push_enabled,
        digest_frequency: row.digest_frequency,
    }
}
