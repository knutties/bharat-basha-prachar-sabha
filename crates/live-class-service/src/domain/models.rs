use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

/// A live class session.
#[derive(Debug, Clone, Serialize)]
pub struct LiveClass {
    pub id: Uuid,
    pub teacher_id: Uuid,
    pub language_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub scheduled_at: DateTime<Utc>,
    pub duration_minutes: i32,
    pub max_students: i32,
    pub status: String,
    pub meeting_url: Option<String>,
}

/// A student enrollment in a live class.
#[derive(Debug, Clone, Serialize)]
pub struct ClassEnrollment {
    pub id: Uuid,
    pub class_id: Uuid,
    pub student_id: Uuid,
    pub enrolled_at: DateTime<Utc>,
    pub attended: bool,
}

/// A teacher's availability slot.
#[derive(Debug, Clone, Serialize)]
pub struct TeacherAvailability {
    pub id: Uuid,
    pub teacher_id: Uuid,
    pub day_of_week: i32,
    pub start_time: String,
    pub end_time: String,
}

/// Input for creating a new live class.
#[derive(Debug)]
pub struct CreateClassInput {
    pub teacher_id: Uuid,
    pub language_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub scheduled_at: DateTime<Utc>,
    pub duration_minutes: i32,
    pub max_students: i32,
}

/// Input for enrolling in a class.
#[derive(Debug)]
pub struct EnrollInput {
    pub class_id: Uuid,
    pub student_id: Uuid,
}

/// Input for a teacher availability slot.
#[derive(Debug)]
pub struct TeacherAvailabilityInput {
    pub teacher_id: Uuid,
    pub day_of_week: i32,
    pub start_time: String,
    pub end_time: String,
}
