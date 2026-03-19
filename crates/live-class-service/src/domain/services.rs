use chrono::Utc;
use uuid::Uuid;

use super::errors::LiveClassError;
use super::models::{
    ClassEnrollment, CreateClassInput, EnrollInput, LiveClass,
    TeacherAvailability, TeacherAvailabilityInput,
};
use super::repository::LiveClassRepository;

/// Core live class service containing all business logic.
pub struct LiveClassServiceImpl<R: LiveClassRepository> {
    repo: R,
}

impl<R: LiveClassRepository> LiveClassServiceImpl<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// Create a new live class.
    pub fn create_class(
        &self,
        input: CreateClassInput,
    ) -> Result<LiveClass, LiveClassError> {
        let class = LiveClass {
            id: Uuid::new_v4(),
            teacher_id: input.teacher_id,
            language_id: input.language_id,
            title: input.title,
            description: input.description,
            scheduled_at: input.scheduled_at,
            duration_minutes: input.duration_minutes,
            max_students: input.max_students,
            status: "scheduled".to_string(),
            meeting_url: None,
        };

        self.repo.create_class(&class)
    }

    /// List all classes.
    pub fn list_classes(&self) -> Result<Vec<LiveClass>, LiveClassError> {
        self.repo.list_classes()
    }

    /// Get a class by ID.
    pub fn get_class(
        &self,
        id: Uuid,
    ) -> Result<LiveClass, LiveClassError> {
        self.repo
            .find_class_by_id(id)?
            .ok_or_else(|| LiveClassError::ClassNotFound(id.to_string()))
    }

    /// Enroll a student in a class.
    pub fn enroll(
        &self,
        input: EnrollInput,
    ) -> Result<ClassEnrollment, LiveClassError> {
        // Verify class exists
        let class = self
            .repo
            .find_class_by_id(input.class_id)?
            .ok_or_else(|| {
                LiveClassError::ClassNotFound(input.class_id.to_string())
            })?;

        // Check if already enrolled
        if self
            .repo
            .find_enrollment(input.class_id, input.student_id)?
            .is_some()
        {
            return Err(LiveClassError::AlreadyEnrolled(
                input.student_id.to_string(),
            ));
        }

        // Check capacity
        let count = self.repo.count_enrollments(input.class_id)?;
        if count >= class.max_students as i64 {
            return Err(LiveClassError::ClassFull(
                input.class_id.to_string(),
            ));
        }

        let enrollment = ClassEnrollment {
            id: Uuid::new_v4(),
            class_id: input.class_id,
            student_id: input.student_id,
            enrolled_at: Utc::now(),
            attended: false,
        };

        self.repo.create_enrollment(&enrollment)
    }

    /// Get teacher availability.
    pub fn get_availability(
        &self,
        teacher_id: Uuid,
    ) -> Result<Vec<TeacherAvailability>, LiveClassError> {
        self.repo.find_availability_by_teacher(teacher_id)
    }

    /// Update teacher availability (replace all slots).
    pub fn update_availability(
        &self,
        teacher_id: Uuid,
        inputs: Vec<TeacherAvailabilityInput>,
    ) -> Result<Vec<TeacherAvailability>, LiveClassError> {
        let slots: Vec<TeacherAvailability> = inputs
            .into_iter()
            .map(|i| TeacherAvailability {
                id: Uuid::new_v4(),
                teacher_id: i.teacher_id,
                day_of_week: i.day_of_week,
                start_time: i.start_time,
                end_time: i.end_time,
            })
            .collect();

        self.repo.replace_availability(teacher_id, &slots)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repository::MockLiveClassRepository;

    #[test]
    fn test_create_class() {
        let mut mock_repo = MockLiveClassRepository::new();
        mock_repo
            .expect_create_class()
            .returning(|c| Ok(c.clone()));

        let service = LiveClassServiceImpl::new(mock_repo);
        let input = CreateClassInput {
            teacher_id: Uuid::new_v4(),
            language_id: Uuid::new_v4(),
            title: "Hindi Basics".to_string(),
            description: Some("Learn Hindi basics".to_string()),
            scheduled_at: Utc::now(),
            duration_minutes: 60,
            max_students: 30,
        };

        let result = service.create_class(input);
        assert!(result.is_ok());
        let class = result.unwrap();
        assert_eq!(class.status, "scheduled");
    }
}
