use uuid::Uuid;

use super::errors::LiveClassError;
use super::models::{ClassEnrollment, LiveClass, TeacherAvailability};

/// Repository trait for live class persistence.
/// Implemented by the database layer; consumed by domain services.
#[cfg_attr(test, mockall::automock)]
pub trait LiveClassRepository: Send + Sync {
    /// Create a new live class.
    fn create_class(
        &self,
        class: &LiveClass,
    ) -> Result<LiveClass, LiveClassError>;

    /// Find a class by ID.
    fn find_class_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<LiveClass>, LiveClassError>;

    /// List all upcoming classes.
    fn list_classes(&self) -> Result<Vec<LiveClass>, LiveClassError>;

    /// Create an enrollment record.
    fn create_enrollment(
        &self,
        enrollment: &ClassEnrollment,
    ) -> Result<ClassEnrollment, LiveClassError>;

    /// Find enrollment by class and student.
    fn find_enrollment(
        &self,
        class_id: Uuid,
        student_id: Uuid,
    ) -> Result<Option<ClassEnrollment>, LiveClassError>;

    /// Count enrollments for a class.
    fn count_enrollments(
        &self,
        class_id: Uuid,
    ) -> Result<i64, LiveClassError>;

    /// Get availability for a teacher.
    fn find_availability_by_teacher(
        &self,
        teacher_id: Uuid,
    ) -> Result<Vec<TeacherAvailability>, LiveClassError>;

    /// Replace all availability slots for a teacher.
    fn replace_availability(
        &self,
        teacher_id: Uuid,
        slots: &[TeacherAvailability],
    ) -> Result<Vec<TeacherAvailability>, LiveClassError>;
}
