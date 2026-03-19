use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

/// A notification sent to a user.
#[derive(Debug, Clone, Serialize)]
pub struct Notification {
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

/// User notification preferences.
#[derive(Debug, Clone, Serialize)]
pub struct NotificationPreference {
    pub user_id: Uuid,
    pub email_enabled: bool,
    pub sms_enabled: bool,
    pub push_enabled: bool,
    pub digest_frequency: Option<String>,
}

/// Input for sending a notification.
#[derive(Debug)]
pub struct SendNotificationInput {
    pub user_id: Uuid,
    pub notification_type: String,
    pub channel: String,
    pub title: String,
    pub body: String,
}

/// Input for updating notification preferences.
#[derive(Debug)]
pub struct UpdatePreferencesInput {
    pub user_id: Uuid,
    pub email_enabled: bool,
    pub sms_enabled: bool,
    pub push_enabled: bool,
    pub digest_frequency: Option<String>,
}
