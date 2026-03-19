use chrono::{DateTime, Utc};
use shared::types::{EducationBoard, Grade, SchoolId, UserRole};
use uuid::Uuid;

/// Core user domain model.
#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub phone: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Student-specific profile data.
#[derive(Debug, Clone)]
pub struct StudentProfile {
    pub user_id: Uuid,
    pub grade: Grade,
    pub mother_tongue: String,
    pub state_of_residence: String,
    pub board: Option<EducationBoard>,
    pub school_id: Option<SchoolId>,
    pub parent_email: Option<String>,
}

/// Teacher-specific profile data.
#[derive(Debug, Clone)]
pub struct TeacherProfile {
    pub user_id: Uuid,
    pub languages: Vec<String>,
    pub qualifications: String,
    pub bio: Option<String>,
    pub verified: bool,
    pub rating: Option<f64>,
}

/// Input for registering a new student.
#[derive(Debug)]
pub struct RegisterStudentInput {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub grade: Grade,
    pub mother_tongue: String,
    pub state_of_residence: String,
    pub board: Option<EducationBoard>,
    pub school_id: Option<SchoolId>,
    pub parent_email: Option<String>,
}

/// Input for registering a new teacher.
#[derive(Debug)]
pub struct RegisterTeacherInput {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub languages: Vec<String>,
    pub qualifications: String,
    pub bio: Option<String>,
}

/// Input for updating a user profile.
#[derive(Debug)]
pub struct UpdateProfileInput {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub grade: Option<Grade>,
    pub state_of_residence: Option<String>,
    pub board: Option<EducationBoard>,
}

/// Authentication token pair.
#[derive(Debug, Clone)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}
