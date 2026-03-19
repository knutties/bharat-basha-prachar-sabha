use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use super::schema::{class_enrollments, live_classes, teacher_availability};

/// Diesel model for reading live classes.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = live_classes)]
pub struct LiveClassRow {
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

/// Diesel model for inserting live classes.
#[derive(Insertable)]
#[diesel(table_name = live_classes)]
pub struct NewLiveClassRow<'a> {
    pub id: Uuid,
    pub teacher_id: Uuid,
    pub language_id: Uuid,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub scheduled_at: DateTime<Utc>,
    pub duration_minutes: i32,
    pub max_students: i32,
    pub status: &'a str,
    pub meeting_url: Option<&'a str>,
}

/// Diesel model for reading class enrollments.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = class_enrollments)]
pub struct ClassEnrollmentRow {
    pub id: Uuid,
    pub class_id: Uuid,
    pub student_id: Uuid,
    pub enrolled_at: DateTime<Utc>,
    pub attended: bool,
}

/// Diesel model for inserting class enrollments.
#[derive(Insertable)]
#[diesel(table_name = class_enrollments)]
pub struct NewClassEnrollmentRow {
    pub id: Uuid,
    pub class_id: Uuid,
    pub student_id: Uuid,
    pub enrolled_at: DateTime<Utc>,
    pub attended: bool,
}

/// Diesel model for reading teacher availability.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = teacher_availability)]
pub struct TeacherAvailabilityRow {
    pub id: Uuid,
    pub teacher_id: Uuid,
    pub day_of_week: i32,
    pub start_time: String,
    pub end_time: String,
}

/// Diesel model for inserting teacher availability.
#[derive(Insertable)]
#[diesel(table_name = teacher_availability)]
pub struct NewTeacherAvailabilityRow<'a> {
    pub id: Uuid,
    pub teacher_id: Uuid,
    pub day_of_week: i32,
    pub start_time: &'a str,
    pub end_time: &'a str,
}
