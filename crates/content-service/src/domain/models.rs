use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

/// A content item created by curriculum authors.
#[derive(Debug, Clone, Serialize)]
pub struct ContentItem {
    pub id: Uuid,
    pub language_id: Uuid,
    pub module_id: Uuid,
    pub lesson_id: Option<Uuid>,
    pub content_type: String,
    pub title: String,
    pub body: String,
    pub media_urls: Option<Vec<String>>,
    pub status: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A media asset uploaded to the system.
#[derive(Debug, Clone, Serialize)]
pub struct MediaAsset {
    pub id: Uuid,
    pub filename: String,
    pub content_type: String,
    pub url: String,
    pub size_bytes: i64,
    pub uploaded_by: Uuid,
    pub uploaded_at: DateTime<Utc>,
}

/// A review of a content item.
#[derive(Debug, Clone, Serialize)]
pub struct ContentReview {
    pub id: Uuid,
    pub content_item_id: Uuid,
    pub reviewer_id: Uuid,
    pub status: String,
    pub comments: Option<String>,
    pub reviewed_at: DateTime<Utc>,
}

/// Input for creating a new content item.
#[derive(Debug)]
pub struct CreateContentInput {
    pub language_id: Uuid,
    pub module_id: Uuid,
    pub lesson_id: Option<Uuid>,
    pub content_type: String,
    pub title: String,
    pub body: String,
    pub media_urls: Option<Vec<String>>,
    pub created_by: Uuid,
}

/// Input for updating a content item.
#[derive(Debug)]
pub struct UpdateContentInput {
    pub title: Option<String>,
    pub body: Option<String>,
    pub content_type: Option<String>,
    pub media_urls: Option<Vec<String>>,
    pub status: Option<String>,
}

/// Input for creating a content review.
#[derive(Debug)]
pub struct CreateReviewInput {
    pub content_item_id: Uuid,
    pub reviewer_id: Uuid,
    pub status: String,
    pub comments: Option<String>,
}

/// Input for uploading a media asset.
#[derive(Debug)]
pub struct UploadMediaInput {
    pub filename: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub uploaded_by: Uuid,
}
