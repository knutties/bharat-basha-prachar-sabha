CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    phone VARCHAR(20) NOT NULL,
    role VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);

CREATE TABLE student_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    grade SMALLINT NOT NULL CHECK (grade >= 1 AND grade <= 12),
    mother_tongue VARCHAR(100) NOT NULL,
    state_of_residence VARCHAR(100) NOT NULL,
    board VARCHAR(50),
    school_id UUID,
    parent_email VARCHAR(255)
);

CREATE INDEX idx_student_profiles_mother_tongue ON student_profiles(mother_tongue);
CREATE INDEX idx_student_profiles_state ON student_profiles(state_of_residence);

CREATE TABLE teacher_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    languages TEXT[] NOT NULL,
    qualifications TEXT NOT NULL,
    bio TEXT,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    rating DOUBLE PRECISION
);

CREATE INDEX idx_teacher_profiles_languages ON teacher_profiles USING GIN(languages);
CREATE INDEX idx_teacher_profiles_verified ON teacher_profiles(verified);
