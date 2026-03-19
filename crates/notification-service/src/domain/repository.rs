use uuid::Uuid;

use super::errors::NotificationError;
use super::models::{Notification, NotificationPreference};

/// Repository trait for notification persistence.
/// Implemented by the database layer; consumed by domain services.
#[cfg_attr(test, mockall::automock)]
pub trait NotificationRepository: Send + Sync {
    /// Create a new notification.
    fn create_notification(
        &self,
        notification: &Notification,
    ) -> Result<Notification, NotificationError>;

    /// Find a notification by ID.
    fn find_notification_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Notification>, NotificationError>;

    /// Get all notifications for a user.
    fn find_notifications_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Notification>, NotificationError>;

    /// Update a notification (e.g., mark as read).
    fn update_notification(
        &self,
        notification: &Notification,
    ) -> Result<Notification, NotificationError>;

    /// Get notification preferences for a user.
    fn find_preferences_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Option<NotificationPreference>, NotificationError>;

    /// Create or update notification preferences.
    fn upsert_preferences(
        &self,
        preferences: &NotificationPreference,
    ) -> Result<NotificationPreference, NotificationError>;
}
