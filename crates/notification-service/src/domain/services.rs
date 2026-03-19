use chrono::Utc;
use uuid::Uuid;

use super::errors::NotificationError;
use super::models::{
    Notification, NotificationPreference, SendNotificationInput,
    UpdatePreferencesInput,
};
use super::repository::NotificationRepository;

const VALID_CHANNELS: &[&str] = &["email", "sms", "push"];

/// Core notification service containing all business logic.
pub struct NotificationServiceImpl<R: NotificationRepository> {
    repo: R,
}

impl<R: NotificationRepository> NotificationServiceImpl<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// Send a notification to a user.
    pub fn send_notification(
        &self,
        input: SendNotificationInput,
    ) -> Result<Notification, NotificationError> {
        // Validate channel
        if !VALID_CHANNELS.contains(&input.channel.as_str()) {
            return Err(NotificationError::InvalidChannel(
                input.channel.clone(),
            ));
        }

        let notification = Notification {
            id: Uuid::new_v4(),
            user_id: input.user_id,
            notification_type: input.notification_type,
            channel: input.channel,
            title: input.title,
            body: input.body,
            status: "sent".to_string(),
            sent_at: Some(Utc::now()),
            read_at: None,
        };

        // TODO: Actually dispatch via email/SMS/push provider
        self.repo.create_notification(&notification)
    }

    /// Get all notifications for a user.
    pub fn get_notifications(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Notification>, NotificationError> {
        self.repo.find_notifications_by_user(user_id)
    }

    /// Mark a notification as read.
    pub fn mark_as_read(
        &self,
        id: Uuid,
    ) -> Result<(), NotificationError> {
        let mut notification = self
            .repo
            .find_notification_by_id(id)?
            .ok_or_else(|| {
                NotificationError::NotificationNotFound(id.to_string())
            })?;

        notification.read_at = Some(Utc::now());
        notification.status = "read".to_string();
        self.repo.update_notification(&notification)?;
        Ok(())
    }

    /// Get notification preferences for a user.
    pub fn get_preferences(
        &self,
        user_id: Uuid,
    ) -> Result<NotificationPreference, NotificationError> {
        self.repo
            .find_preferences_by_user(user_id)?
            .ok_or_else(|| {
                // Return defaults if no preferences exist
                NotificationError::UserNotFound(user_id.to_string())
            })
    }

    /// Update notification preferences for a user.
    pub fn update_preferences(
        &self,
        input: UpdatePreferencesInput,
    ) -> Result<NotificationPreference, NotificationError> {
        let prefs = NotificationPreference {
            user_id: input.user_id,
            email_enabled: input.email_enabled,
            sms_enabled: input.sms_enabled,
            push_enabled: input.push_enabled,
            digest_frequency: input.digest_frequency,
        };

        self.repo.upsert_preferences(&prefs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repository::MockNotificationRepository;

    #[test]
    fn test_send_notification_valid_channel() {
        let mut mock_repo = MockNotificationRepository::new();
        mock_repo
            .expect_create_notification()
            .returning(|n| Ok(n.clone()));

        let service = NotificationServiceImpl::new(mock_repo);
        let input = SendNotificationInput {
            user_id: Uuid::new_v4(),
            notification_type: "reminder".to_string(),
            channel: "email".to_string(),
            title: "Test".to_string(),
            body: "Test body".to_string(),
        };

        let result = service.send_notification(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_send_notification_invalid_channel() {
        let mock_repo = MockNotificationRepository::new();
        let service = NotificationServiceImpl::new(mock_repo);

        let input = SendNotificationInput {
            user_id: Uuid::new_v4(),
            notification_type: "reminder".to_string(),
            channel: "pigeon".to_string(),
            title: "Test".to_string(),
            body: "Test body".to_string(),
        };

        let result = service.send_notification(input);
        assert!(matches!(
            result,
            Err(NotificationError::InvalidChannel(_))
        ));
    }
}
