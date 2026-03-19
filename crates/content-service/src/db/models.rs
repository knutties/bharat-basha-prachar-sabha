use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use super::schema::{content_items, content_reviews, media_assets};

/// Diesel model for reading content items.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = content_items)]
pub struct ContentItemRow {
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

/// Diesel model for inserting content items.
#[derive(Insertable)]
#[diesel(table_name = content_items)]
pub struct NewContentItemRow<'a> {
    pub id: Uuid,
    pub language_id: Uuid,
    pub module_id: Uuid,
    pub lesson_id: Option<Uuid>,
    pub content_type: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    pub media_urls: Option<&'a [String]>,
    pub status: &'a str,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Diesel model for reading media assets.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = media_assets)]
pub struct MediaAssetRow {
    pub id: Uuid,
    pub filename: String,
    pub content_type: String,
    pub url: String,
    pub size_bytes: i64,
    pub uploaded_by: Uuid,
    pub uploaded_at: DateTime<Utc>,
}

/// Diesel model for inserting media assets.
#[derive(Insertable)]
#[diesel(table_name = media_assets)]
pub struct NewMediaAssetRow<'a> {
    pub id: Uuid,
    pub filename: &'a str,
    pub content_type: &'a str,
    pub url: &'a str,
    pub size_bytes: i64,
    pub uploaded_by: Uuid,
    pub uploaded_at: DateTime<Utc>,
}

/// Diesel model for reading content reviews.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = content_reviews)]
pub struct ContentReviewRow {
    pub id: Uuid,
    pub content_item_id: Uuid,
    pub reviewer_id: Uuid,
    pub status: String,
    pub comments: Option<String>,
    pub reviewed_at: DateTime<Utc>,
}

/// Diesel model for inserting content reviews.
#[derive(Insertable)]
#[diesel(table_name = content_reviews)]
pub struct NewContentReviewRow<'a> {
    pub id: Uuid,
    pub content_item_id: Uuid,
    pub reviewer_id: Uuid,
    pub status: &'a str,
    pub comments: Option<&'a str>,
    pub reviewed_at: DateTime<Utc>,
}
