use uuid::Uuid;

use super::errors::ContentError;
use super::models::{ContentItem, ContentReview, MediaAsset};

/// Repository trait for content persistence.
/// Implemented by the database layer; consumed by domain services.
#[cfg_attr(test, mockall::automock)]
pub trait ContentRepository: Send + Sync {
    /// Create a new content item.
    fn create_content_item(
        &self,
        item: &ContentItem,
    ) -> Result<ContentItem, ContentError>;

    /// Find a content item by ID.
    fn find_content_item_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<ContentItem>, ContentError>;

    /// Update a content item.
    fn update_content_item(
        &self,
        item: &ContentItem,
    ) -> Result<ContentItem, ContentError>;

    /// List content items with optional filters.
    fn list_content_items(
        &self,
        language_id: Option<Uuid>,
        module_id: Option<Uuid>,
        status: Option<&str>,
        content_type: Option<&str>,
    ) -> Result<Vec<ContentItem>, ContentError>;

    /// Create a media asset record.
    fn create_media_asset(
        &self,
        asset: &MediaAsset,
    ) -> Result<MediaAsset, ContentError>;

    /// Find a media asset by ID.
    fn find_media_asset_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<MediaAsset>, ContentError>;

    /// Create a content review.
    fn create_content_review(
        &self,
        review: &ContentReview,
    ) -> Result<ContentReview, ContentError>;

    /// Get reviews for a content item.
    fn find_reviews_by_content_item(
        &self,
        content_item_id: Uuid,
    ) -> Result<Vec<ContentReview>, ContentError>;
}
