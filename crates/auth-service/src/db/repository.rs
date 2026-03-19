use diesel::prelude::*;
use shared::types::{Grade, SchoolId, UserRole};
use uuid::Uuid;

use super::connection::DbPool;
use super::models::*;
use super::schema::{student_profiles, teacher_profiles, users};
use crate::domain::errors::AuthError;
use crate::domain::models::{
    StudentProfile, TeacherProfile, UpdateProfileInput, User,
};
use crate::domain::repository::UserRepository;

/// PostgreSQL implementation of the UserRepository trait.
pub struct PgUserRepository {
    pool: DbPool,
}

impl PgUserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn get_conn(
        &self,
    ) -> Result<
        diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<PgConnection>,
        >,
        AuthError,
    > {
        self.pool
            .get()
            .map_err(|e| AuthError::RepositoryError(e.to_string()))
    }
}

impl UserRepository for PgUserRepository {
    fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AuthError> {
        let mut conn = self.get_conn()?;

        let row = users::table
            .filter(users::id.eq(id))
            .select(UserRow::as_select())
            .first::<UserRow>(&mut conn)
            .optional()
            .map_err(|e| AuthError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_user))
    }

    fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError> {
        let mut conn = self.get_conn()?;

        let row = users::table
            .filter(users::email.eq(email))
            .select(UserRow::as_select())
            .first::<UserRow>(&mut conn)
            .optional()
            .map_err(|e| AuthError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_user))
    }

    fn create_user(&self, user: &User) -> Result<User, AuthError> {
        let mut conn = self.get_conn()?;

        let new_row = NewUserRow {
            id: user.id,
            name: &user.name,
            email: &user.email,
            password_hash: &user.password_hash,
            phone: &user.phone,
            role: &format!("{:?}", user.role),
            created_at: user.created_at,
            updated_at: user.updated_at,
        };

        let row = diesel::insert_into(users::table)
            .values(&new_row)
            .returning(UserRow::as_returning())
            .get_result::<UserRow>(&mut conn)
            .map_err(|e| AuthError::RepositoryError(e.to_string()))?;

        Ok(row_to_user(row))
    }

    fn update_user(
        &self,
        id: Uuid,
        input: &UpdateProfileInput,
    ) -> Result<(), AuthError> {
        let mut conn = self.get_conn()?;

        // Build dynamic update — only update fields that are Some
        if let Some(ref name) = input.name {
            diesel::update(users::table.filter(users::id.eq(id)))
                .set(users::name.eq(name))
                .execute(&mut conn)
                .map_err(|e| AuthError::RepositoryError(e.to_string()))?;
        }

        if let Some(ref phone) = input.phone {
            diesel::update(users::table.filter(users::id.eq(id)))
                .set(users::phone.eq(phone))
                .execute(&mut conn)
                .map_err(|e| AuthError::RepositoryError(e.to_string()))?;
        }

        Ok(())
    }

    fn create_student_profile(
        &self,
        profile: &StudentProfile,
    ) -> Result<(), AuthError> {
        let mut conn = self.get_conn()?;

        let new_row = NewStudentProfileRow {
            user_id: profile.user_id,
            grade: profile.grade.value() as i16,
            mother_tongue: &profile.mother_tongue,
            state_of_residence: &profile.state_of_residence,
            board: None, // TODO: serialize EducationBoard
            school_id: profile.school_id.map(|s| s.0),
            parent_email: profile.parent_email.as_deref(),
        };

        diesel::insert_into(student_profiles::table)
            .values(&new_row)
            .execute(&mut conn)
            .map_err(|e| AuthError::RepositoryError(e.to_string()))?;

        Ok(())
    }

    fn create_teacher_profile(
        &self,
        profile: &TeacherProfile,
    ) -> Result<(), AuthError> {
        let mut conn = self.get_conn()?;

        let languages: Vec<Option<String>> =
            profile.languages.iter().map(|l| Some(l.clone())).collect();
        let new_row = NewTeacherProfileRow {
            user_id: profile.user_id,
            languages: &languages,
            qualifications: &profile.qualifications,
            bio: profile.bio.as_deref(),
            verified: profile.verified,
            rating: profile.rating,
        };

        diesel::insert_into(teacher_profiles::table)
            .values(&new_row)
            .execute(&mut conn)
            .map_err(|e| AuthError::RepositoryError(e.to_string()))?;

        Ok(())
    }

    fn find_student_profile(
        &self,
        user_id: Uuid,
    ) -> Result<Option<StudentProfile>, AuthError> {
        let mut conn = self.get_conn()?;

        let row = student_profiles::table
            .filter(student_profiles::user_id.eq(user_id))
            .select(StudentProfileRow::as_select())
            .first::<StudentProfileRow>(&mut conn)
            .optional()
            .map_err(|e| AuthError::RepositoryError(e.to_string()))?;

        Ok(row.map(|r| StudentProfile {
            user_id: r.user_id,
            grade: Grade::new(r.grade as u8).unwrap_or(Grade::new(1).unwrap()),
            mother_tongue: r.mother_tongue,
            state_of_residence: r.state_of_residence,
            board: None, // TODO: deserialize
            school_id: r.school_id.map(SchoolId),
            parent_email: r.parent_email,
        }))
    }

    fn find_teacher_profile(
        &self,
        user_id: Uuid,
    ) -> Result<Option<TeacherProfile>, AuthError> {
        let mut conn = self.get_conn()?;

        let row = teacher_profiles::table
            .filter(teacher_profiles::user_id.eq(user_id))
            .select(TeacherProfileRow::as_select())
            .first::<TeacherProfileRow>(&mut conn)
            .optional()
            .map_err(|e| AuthError::RepositoryError(e.to_string()))?;

        Ok(row.map(|r| TeacherProfile {
            user_id: r.user_id,
            languages: r.languages.into_iter().flatten().collect(),
            qualifications: r.qualifications,
            bio: r.bio,
            verified: r.verified,
            rating: r.rating,
        }))
    }
}

/// Convert a database row to a domain User model.
fn row_to_user(row: UserRow) -> User {
    let role = match row.role.as_str() {
        "Student" => UserRole::Student,
        "Parent" => UserRole::Parent,
        "Teacher" => UserRole::Teacher,
        "SchoolAdmin" => UserRole::SchoolAdmin,
        "PlatformAdmin" => UserRole::PlatformAdmin,
        _ => UserRole::Student,
    };

    User {
        id: row.id,
        name: row.name,
        email: row.email,
        password_hash: row.password_hash,
        phone: row.phone,
        role,
        created_at: row.created_at,
        updated_at: row.updated_at,
    }
}
