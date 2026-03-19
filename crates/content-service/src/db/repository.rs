use diesel::prelude::*;
use uuid::Uuid;

use super::connection::DbPool;
use super::models::*;
use super::schema::{content_items, content_reviews, media_assets};
use crate::domain::errors::ContentError;
use crate::domain::models::{ContentItem, ContentReview, MediaAsset};
use crate::domain::repository::ContentRepository;

/// PostgreSQL implementation of the ContentRepository trait.
pub struct PgContentRepository {
    pool: DbPool,
}

impl PgContentRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn get_conn(
        &self,
    ) -> Result<
        diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<PgConnection>,
        >,
        ContentError,
    > {
        self.pool
            .get()
            .map_err(|e| ContentError::RepositoryError(e.to_string()))
    }
}

impl ContentRepository for PgContentRepository {
    fn create_content_item(
        &self,
        item: &ContentItem,
    ) -> Result<ContentItem, ContentError> {
        let mut conn = self.get_conn()?;

        let new_row = NewContentItemRow {
            id: item.id,
            language_id: item.language_id,
            module_id: item.module_id,
            lesson_id: item.lesson_id,
            content_type: &item.content_type,
            title: &item.title,
            body: &item.body,
            media_urls: item.media_urls.as_deref(),
            status: &item.status,
            created_by: item.created_by,
            created_at: item.created_at,
            updated_at: item.updated_at,
        };

        let row = diesel::insert_into(content_items::table)
            .values(&new_row)
            .returning(ContentItemRow::as_returning())
            .get_result::<ContentItemRow>(&mut conn)
            .map_err(|e| ContentError::RepositoryError(e.to_string()))?;

        Ok(row_to_content_item(row))
    }

    fn find_content_item_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<ContentItem>, ContentError> {
        let mut conn = self.get_conn()?;

        let row = content_items::table
            .filter(content_items::id.eq(id))
            .select(ContentItemRow::as_select())
            .first::<ContentItemRow>(&mut conn)
            .optional()
            .map_err(|e| ContentError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_content_item))
    }

    fn update_content_item(
        &self,
        item: &ContentItem,
    ) -> Result<ContentItem, ContentError> {
        let mut conn = self.get_conn()?;

        diesel::update(
            content_items::table.filter(content_items::id.eq(item.id)),
        )
        .set((
            content_items::title.eq(&item.title),
            content_items::body.eq(&item.body),
            content_items::content_type.eq(&item.content_type),
            content_items::status.eq(&item.status),
            content_items::updated_at.eq(item.updated_at),
        ))
        .execute(&mut conn)
        .map_err(|e| ContentError::RepositoryError(e.to_string()))?;

        self.find_content_item_by_id(item.id)?
            .ok_or_else(|| {
                ContentError::ContentNotFound(item.id.to_string())
            })
    }

    fn list_content_items(
        &self,
        language_id: Option<Uuid>,
        module_id: Option<Uuid>,
        status: Option<&str>,
        content_type: Option<&str>,
    ) -> Result<Vec<ContentItem>, ContentError> {
        let mut conn = self.get_conn()?;

        let mut query = content_items::table
            .into_boxed();

        if let Some(lang_id) = language_id {
            query = query.filter(content_items::language_id.eq(lang_id));
        }
        if let Some(mod_id) = module_id {
            query = query.filter(content_items::module_id.eq(mod_id));
        }
        if let Some(s) = status {
            query = query.filter(content_items::status.eq(s));
        }
        if let Some(ct) = content_type {
            query = query.filter(content_items::content_type.eq(ct));
        }

        let rows = query
            .select(ContentItemRow::as_select())
            .load::<ContentItemRow>(&mut conn)
            .map_err(|e| ContentError::RepositoryError(e.to_string()))?;

        Ok(rows.into_iter().map(row_to_content_item).collect())
    }

    fn create_media_asset(
        &self,
        asset: &MediaAsset,
    ) -> Result<MediaAsset, ContentError> {
        let mut conn = self.get_conn()?;

        let new_row = NewMediaAssetRow {
            id: asset.id,
            filename: &asset.filename,
            content_type: &asset.content_type,
            url: &asset.url,
            size_bytes: asset.size_bytes,
            uploaded_by: asset.uploaded_by,
            uploaded_at: asset.uploaded_at,
        };

        let row = diesel::insert_into(media_assets::table)
            .values(&new_row)
            .returning(MediaAssetRow::as_returning())
            .get_result::<MediaAssetRow>(&mut conn)
            .map_err(|e| ContentError::RepositoryError(e.to_string()))?;

        Ok(row_to_media_asset(row))
    }

    fn find_media_asset_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<MediaAsset>, ContentError> {
        let mut conn = self.get_conn()?;

        let row = media_assets::table
            .filter(media_assets::id.eq(id))
            .select(MediaAssetRow::as_select())
            .first::<MediaAssetRow>(&mut conn)
            .optional()
            .map_err(|e| ContentError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_media_asset))
    }

    fn create_content_review(
        &self,
        review: &ContentReview,
    ) -> Result<ContentReview, ContentError> {
        let mut conn = self.get_conn()?;

        let new_row = NewContentReviewRow {
            id: review.id,
            content_item_id: review.content_item_id,
            reviewer_id: review.reviewer_id,
            status: &review.status,
            comments: review.comments.as_deref(),
            reviewed_at: review.reviewed_at,
        };

        let row = diesel::insert_into(content_reviews::table)
            .values(&new_row)
            .returning(ContentReviewRow::as_returning())
            .get_result::<ContentReviewRow>(&mut conn)
            .map_err(|e| ContentError::RepositoryError(e.to_string()))?;

        Ok(row_to_content_review(row))
    }

    fn find_reviews_by_content_item(
        &self,
        content_item_id: Uuid,
    ) -> Result<Vec<ContentReview>, ContentError> {
        let mut conn = self.get_conn()?;

        let rows = content_reviews::table
            .filter(content_reviews::content_item_id.eq(content_item_id))
            .select(ContentReviewRow::as_select())
            .load::<ContentReviewRow>(&mut conn)
            .map_err(|e| ContentError::RepositoryError(e.to_string()))?;

        Ok(rows.into_iter().map(row_to_content_review).collect())
    }
}

fn row_to_content_item(row: ContentItemRow) -> ContentItem {
    ContentItem {
        id: row.id,
        language_id: row.language_id,
        module_id: row.module_id,
        lesson_id: row.lesson_id,
        content_type: row.content_type,
        title: row.title,
        body: row.body,
        media_urls: row.media_urls,
        status: row.status,
        created_by: row.created_by,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}

fn row_to_media_asset(row: MediaAssetRow) -> MediaAsset {
    MediaAsset {
        id: row.id,
        filename: row.filename,
        content_type: row.content_type,
        url: row.url,
        size_bytes: row.size_bytes,
        uploaded_by: row.uploaded_by,
        uploaded_at: row.uploaded_at,
    }
}

fn row_to_content_review(row: ContentReviewRow) -> ContentReview {
    ContentReview {
        id: row.id,
        content_item_id: row.content_item_id,
        reviewer_id: row.reviewer_id,
        status: row.status,
        comments: row.comments,
        reviewed_at: row.reviewed_at,
    }
}
