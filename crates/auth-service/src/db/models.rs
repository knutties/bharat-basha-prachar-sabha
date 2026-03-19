use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use super::schema::{student_profiles, teacher_profiles, users};

/// Diesel model for reading users from the database.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = users)]
pub struct UserRow {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub phone: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Diesel model for inserting new users.
#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUserRow<'a> {
    pub id: Uuid,
    pub name: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
    pub phone: &'a str,
    pub role: &'a str,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Diesel model for reading student profiles.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = student_profiles)]
#[allow(dead_code)]
pub struct StudentProfileRow {
    pub user_id: Uuid,
    pub grade: i16,
    pub mother_tongue: String,
    pub state_of_residence: String,
    pub board: Option<String>,
    pub school_id: Option<Uuid>,
    pub parent_email: Option<String>,
}

/// Diesel model for inserting student profiles.
#[derive(Insertable)]
#[diesel(table_name = student_profiles)]
pub struct NewStudentProfileRow<'a> {
    pub user_id: Uuid,
    pub grade: i16,
    pub mother_tongue: &'a str,
    pub state_of_residence: &'a str,
    pub board: Option<&'a str>,
    pub school_id: Option<Uuid>,
    pub parent_email: Option<&'a str>,
}

/// Diesel model for reading teacher profiles.
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = teacher_profiles)]
#[allow(dead_code)]
pub struct TeacherProfileRow {
    pub user_id: Uuid,
    pub languages: Vec<Option<String>>,
    pub qualifications: String,
    pub bio: Option<String>,
    pub verified: bool,
    pub rating: Option<f64>,
}

/// Diesel model for inserting teacher profiles.
#[derive(Insertable)]
#[diesel(table_name = teacher_profiles)]
pub struct NewTeacherProfileRow<'a> {
    pub user_id: Uuid,
    pub languages: &'a [Option<String>],
    pub qualifications: &'a str,
    pub bio: Option<&'a str>,
    pub verified: bool,
    pub rating: Option<f64>,
}
