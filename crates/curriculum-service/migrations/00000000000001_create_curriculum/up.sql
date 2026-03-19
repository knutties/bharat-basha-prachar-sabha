CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Languages available on the platform.
CREATE TABLE languages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL UNIQUE,
    script VARCHAR(100) NOT NULL,
    iso_code VARCHAR(10) NOT NULL UNIQUE,
    family VARCHAR(100) NOT NULL
);

CREATE INDEX idx_languages_iso_code ON languages(iso_code);
CREATE INDEX idx_languages_family ON languages(family);

-- Curricula: one per language + grade combination.
CREATE TABLE curricula (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    language_id UUID NOT NULL REFERENCES languages(id) ON DELETE CASCADE,
    grade SMALLINT NOT NULL CHECK (grade >= 1 AND grade <= 12),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    UNIQUE (language_id, grade)
);

CREATE INDEX idx_curricula_language_id ON curricula(language_id);
CREATE INDEX idx_curricula_grade ON curricula(grade);

-- Modules within a curriculum, each targeting a skill type.
CREATE TABLE modules (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    curriculum_id UUID NOT NULL REFERENCES curricula(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    skill_type VARCHAR(50) NOT NULL,
    order_index INTEGER NOT NULL,
    estimated_minutes INTEGER NOT NULL DEFAULT 0,
    UNIQUE (curriculum_id, order_index)
);

CREATE INDEX idx_modules_curriculum_id ON modules(curriculum_id);
CREATE INDEX idx_modules_skill_type ON modules(skill_type);

-- Lessons within a module.
CREATE TABLE lessons (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    module_id UUID NOT NULL REFERENCES modules(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    lesson_type VARCHAR(50) NOT NULL,
    content TEXT NOT NULL DEFAULT '{}',
    media_urls VARCHAR(500)[] NOT NULL DEFAULT '{}',
    duration_minutes INTEGER NOT NULL DEFAULT 0,
    order_index INTEGER NOT NULL,
    UNIQUE (module_id, order_index)
);

CREATE INDEX idx_lessons_module_id ON lessons(module_id);
CREATE INDEX idx_lessons_lesson_type ON lessons(lesson_type);

-- Student enrollments in a language course.
CREATE TABLE enrollments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID NOT NULL,
    language_id UUID NOT NULL REFERENCES languages(id) ON DELETE CASCADE,
    grade SMALLINT NOT NULL CHECK (grade >= 1 AND grade <= 12),
    proficiency VARCHAR(50) NOT NULL DEFAULT 'Beginner',
    enrolled_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (student_id, language_id)
);

CREATE INDEX idx_enrollments_student_id ON enrollments(student_id);
CREATE INDEX idx_enrollments_language_id ON enrollments(language_id);

-- Student progress on individual lessons.
CREATE TABLE progress (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID NOT NULL,
    lesson_id UUID NOT NULL REFERENCES lessons(id) ON DELETE CASCADE,
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    score DOUBLE PRECISION,
    time_spent_seconds INTEGER NOT NULL DEFAULT 0,
    UNIQUE (student_id, lesson_id)
);

CREATE INDEX idx_progress_student_id ON progress(student_id);
CREATE INDEX idx_progress_lesson_id ON progress(lesson_id);
CREATE INDEX idx_progress_student_lesson ON progress(student_id, lesson_id);
