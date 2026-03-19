CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE live_classes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    teacher_id UUID NOT NULL,
    language_id UUID NOT NULL,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    scheduled_at TIMESTAMPTZ NOT NULL,
    duration_minutes INTEGER NOT NULL DEFAULT 60,
    max_students INTEGER NOT NULL DEFAULT 30,
    status VARCHAR(50) NOT NULL DEFAULT 'scheduled',
    meeting_url VARCHAR(1000)
);

CREATE INDEX idx_live_classes_teacher_id ON live_classes(teacher_id);
CREATE INDEX idx_live_classes_language_id ON live_classes(language_id);
CREATE INDEX idx_live_classes_scheduled_at ON live_classes(scheduled_at);
CREATE INDEX idx_live_classes_status ON live_classes(status);

CREATE TABLE class_enrollments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    class_id UUID NOT NULL REFERENCES live_classes(id) ON DELETE CASCADE,
    student_id UUID NOT NULL,
    enrolled_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    attended BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE(class_id, student_id)
);

CREATE INDEX idx_class_enrollments_class_id ON class_enrollments(class_id);
CREATE INDEX idx_class_enrollments_student_id ON class_enrollments(student_id);

CREATE TABLE teacher_availability (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    teacher_id UUID NOT NULL,
    day_of_week INTEGER NOT NULL CHECK (day_of_week >= 0 AND day_of_week <= 6),
    start_time VARCHAR(10) NOT NULL,
    end_time VARCHAR(10) NOT NULL
);

CREATE INDEX idx_teacher_availability_teacher_id ON teacher_availability(teacher_id);
