use uuid::Uuid;

use super::errors::AuthError;
use super::models::{StudentProfile, TeacherProfile, UpdateProfileInput, User};

/// Repository trait for user persistence.
/// Implemented by the database layer; consumed by domain services.
/// This trait has no knowledge of Diesel, SQL, or any infrastructure.
#[cfg_attr(test, mockall::automock)]
pub trait UserRepository: Send + Sync {
    /// Find a user by their unique ID.
    fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AuthError>;

    /// Find a user by email address.
    fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError>;

    /// Insert a new user and return the created user.
    fn create_user(&self, user: &User) -> Result<User, AuthError>;

    /// Update an existing user's profile fields.
    fn update_user(
        &self,
        id: Uuid,
        input: &UpdateProfileInput,
    ) -> Result<(), AuthError>;

    /// Insert a student profile linked to a user.
    fn create_student_profile(
        &self,
        profile: &StudentProfile,
    ) -> Result<(), AuthError>;

    /// Insert a teacher profile linked to a user.
    fn create_teacher_profile(
        &self,
        profile: &TeacherProfile,
    ) -> Result<(), AuthError>;

    /// Get student profile for a user.
    fn find_student_profile(
        &self,
        user_id: Uuid,
    ) -> Result<Option<StudentProfile>, AuthError>;

    /// Get teacher profile for a user.
    fn find_teacher_profile(
        &self,
        user_id: Uuid,
    ) -> Result<Option<TeacherProfile>, AuthError>;
}
