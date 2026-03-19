use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use super::schema::{notification_preferences, notifications};

/// Diesel model for reading notifications.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = notifications)]
pub struct NotificationRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: String,
    pub channel: String,
    pub title: String,
    pub body: String,
    pub status: String,
    pub sent_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
}

/// Diesel model for inserting notifications.
#[derive(Insertable)]
#[diesel(table_name = notifications)]
pub struct NewNotificationRow<'a> {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: &'a str,
    pub channel: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    pub status: &'a str,
    pub sent_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
}

/// Diesel model for reading notification preferences.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = notification_preferences)]
pub struct NotificationPreferenceRow {
    pub user_id: Uuid,
    pub email_enabled: bool,
    pub sms_enabled: bool,
    pub push_enabled: bool,
    pub digest_frequency: Option<String>,
}

/// Diesel model for inserting notification preferences.
#[derive(Insertable)]
#[diesel(table_name = notification_preferences)]
pub struct NewNotificationPreferenceRow<'a> {
    pub user_id: Uuid,
    pub email_enabled: bool,
    pub sms_enabled: bool,
    pub push_enabled: bool,
    pub digest_frequency: Option<&'a str>,
}
