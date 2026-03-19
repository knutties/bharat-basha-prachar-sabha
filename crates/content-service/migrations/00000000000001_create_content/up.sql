CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE content_items (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    language_id UUID NOT NULL,
    module_id UUID NOT NULL,
    lesson_id UUID,
    content_type VARCHAR(50) NOT NULL,
    title VARCHAR(500) NOT NULL,
    body TEXT NOT NULL,
    media_urls VARCHAR(500)[],
    status VARCHAR(50) NOT NULL DEFAULT 'draft',
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_content_items_language_id ON content_items(language_id);
CREATE INDEX idx_content_items_module_id ON content_items(module_id);
CREATE INDEX idx_content_items_status ON content_items(status);
CREATE INDEX idx_content_items_content_type ON content_items(content_type);
CREATE INDEX idx_content_items_created_by ON content_items(created_by);

CREATE TABLE media_assets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    filename VARCHAR(500) NOT NULL,
    content_type VARCHAR(100) NOT NULL,
    url VARCHAR(1000) NOT NULL,
    size_bytes BIGINT NOT NULL,
    uploaded_by UUID NOT NULL,
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_media_assets_uploaded_by ON media_assets(uploaded_by);

CREATE TABLE content_reviews (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    content_item_id UUID NOT NULL REFERENCES content_items(id) ON DELETE CASCADE,
    reviewer_id UUID NOT NULL,
    status VARCHAR(50) NOT NULL,
    comments TEXT,
    reviewed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_content_reviews_content_item_id ON content_reviews(content_item_id);
CREATE INDEX idx_content_reviews_reviewer_id ON content_reviews(reviewer_id);
