CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE assessments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID NOT NULL,
    module_id UUID NOT NULL,
    assessment_type VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'in_progress',
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    submitted_at TIMESTAMPTZ,
    time_limit_minutes INTEGER
);

CREATE INDEX idx_assessments_student_id ON assessments(student_id);
CREATE INDEX idx_assessments_module_id ON assessments(module_id);
CREATE INDEX idx_assessments_status ON assessments(status);

CREATE TABLE questions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    assessment_id UUID NOT NULL REFERENCES assessments(id) ON DELETE CASCADE,
    question_text TEXT NOT NULL,
    question_type VARCHAR(50) NOT NULL,
    options TEXT,
    media_url VARCHAR(500),
    max_score INTEGER NOT NULL DEFAULT 1,
    correct_answer TEXT NOT NULL,
    order_index INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_questions_assessment_id ON questions(assessment_id);

CREATE TABLE student_answers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    assessment_id UUID NOT NULL REFERENCES assessments(id) ON DELETE CASCADE,
    question_id UUID NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    answer TEXT NOT NULL,
    score INTEGER,
    is_correct BOOLEAN
);

CREATE INDEX idx_student_answers_assessment_id ON student_answers(assessment_id);
CREATE INDEX idx_student_answers_question_id ON student_answers(question_id);

CREATE TABLE credits (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID NOT NULL,
    language_id UUID NOT NULL,
    grade INTEGER NOT NULL,
    semester INTEGER NOT NULL,
    score DOUBLE PRECISION NOT NULL,
    grade_letter VARCHAR(5) NOT NULL,
    credit_points DOUBLE PRECISION NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'issued',
    certificate_url VARCHAR(500),
    verification_code VARCHAR(100) NOT NULL UNIQUE,
    issued_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_credits_student_id ON credits(student_id);
CREATE INDEX idx_credits_language_id ON credits(language_id);
CREATE UNIQUE INDEX idx_credits_verification_code ON credits(verification_code);
