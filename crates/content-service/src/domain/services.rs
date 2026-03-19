use chrono::Utc;
use uuid::Uuid;

use super::errors::ContentError;
use super::models::{
    ContentItem, ContentReview, CreateContentInput, CreateReviewInput,
    MediaAsset, UpdateContentInput, UploadMediaInput,
};
use super::repository::ContentRepository;

/// Core content service containing all business logic.
pub struct ContentServiceImpl<R: ContentRepository> {
    repo: R,
}

impl<R: ContentRepository> ContentServiceImpl<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// Create a new content item.
    pub fn create_content(
        &self,
        input: CreateContentInput,
    ) -> Result<ContentItem, ContentError> {
        let now = Utc::now();
        let item = ContentItem {
            id: Uuid::new_v4(),
            language_id: input.language_id,
            module_id: input.module_id,
            lesson_id: input.lesson_id,
            content_type: input.content_type,
            title: input.title,
            body: input.body,
            media_urls: input.media_urls,
            status: "draft".to_string(),
            created_by: input.created_by,
            created_at: now,
            updated_at: now,
        };

        self.repo.create_content_item(&item)
    }

    /// Get a content item by ID.
    pub fn get_content(
        &self,
        id: Uuid,
    ) -> Result<ContentItem, ContentError> {
        self.repo
            .find_content_item_by_id(id)?
            .ok_or_else(|| ContentError::ContentNotFound(id.to_string()))
    }

    /// Update a content item.
    pub fn update_content(
        &self,
        id: Uuid,
        input: UpdateContentInput,
    ) -> Result<ContentItem, ContentError> {
        let mut item = self
            .repo
            .find_content_item_by_id(id)?
            .ok_or_else(|| ContentError::ContentNotFound(id.to_string()))?;

        if let Some(title) = input.title {
            item.title = title;
        }
        if let Some(body) = input.body {
            item.body = body;
        }
        if let Some(content_type) = input.content_type {
            item.content_type = content_type;
        }
        if let Some(media_urls) = input.media_urls {
            item.media_urls = Some(media_urls);
        }
        if let Some(status) = input.status {
            item.status = status;
        }
        item.updated_at = Utc::now();

        self.repo.update_content_item(&item)
    }

    /// List content items with filters.
    pub fn list_content(
        &self,
        language_id: Option<Uuid>,
        module_id: Option<Uuid>,
        status: Option<&str>,
        content_type: Option<&str>,
    ) -> Result<Vec<ContentItem>, ContentError> {
        self.repo
            .list_content_items(language_id, module_id, status, content_type)
    }

    /// Create a review for a content item.
    pub fn create_review(
        &self,
        input: CreateReviewInput,
    ) -> Result<ContentReview, ContentError> {
        // Verify content item exists
        self.repo
            .find_content_item_by_id(input.content_item_id)?
            .ok_or_else(|| {
                ContentError::ContentNotFound(
                    input.content_item_id.to_string(),
                )
            })?;

        let review = ContentReview {
            id: Uuid::new_v4(),
            content_item_id: input.content_item_id,
            reviewer_id: input.reviewer_id,
            status: input.status,
            comments: input.comments,
            reviewed_at: Utc::now(),
        };

        self.repo.create_content_review(&review)
    }

    /// Upload a media asset (creates the record; actual upload is handled externally).
    pub fn upload_media(
        &self,
        input: UploadMediaInput,
    ) -> Result<MediaAsset, ContentError> {
        let asset = MediaAsset {
            id: Uuid::new_v4(),
            filename: input.filename.clone(),
            content_type: input.content_type,
            url: format!("/media/{}/{}", Uuid::new_v4(), input.filename),
            size_bytes: input.size_bytes,
            uploaded_by: input.uploaded_by,
            uploaded_at: Utc::now(),
        };

        self.repo.create_media_asset(&asset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repository::MockContentRepository;

    #[test]
    fn test_create_content() {
        let mut mock_repo = MockContentRepository::new();
        mock_repo
            .expect_create_content_item()
            .returning(|item| Ok(item.clone()));

        let service = ContentServiceImpl::new(mock_repo);
        let input = CreateContentInput {
            language_id: Uuid::new_v4(),
            module_id: Uuid::new_v4(),
            lesson_id: None,
            content_type: "text".to_string(),
            title: "Hello World".to_string(),
            body: "Content body".to_string(),
            media_urls: None,
            created_by: Uuid::new_v4(),
        };

        let result = service.create_content(input);
        assert!(result.is_ok());
        let item = result.unwrap();
        assert_eq!(item.status, "draft");
    }
}
