use diesel::prelude::*;
use uuid::Uuid;

use super::connection::DbPool;
use super::models::*;
use super::schema::{class_enrollments, live_classes, teacher_availability};
use crate::domain::errors::LiveClassError;
use crate::domain::models::{
    ClassEnrollment, LiveClass, TeacherAvailability,
};
use crate::domain::repository::LiveClassRepository;

/// PostgreSQL implementation of the LiveClassRepository trait.
pub struct PgLiveClassRepository {
    pool: DbPool,
}

impl PgLiveClassRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    fn get_conn(
        &self,
    ) -> Result<
        diesel::r2d2::PooledConnection<
            diesel::r2d2::ConnectionManager<PgConnection>,
        >,
        LiveClassError,
    > {
        self.pool
            .get()
            .map_err(|e| LiveClassError::RepositoryError(e.to_string()))
    }
}

impl LiveClassRepository for PgLiveClassRepository {
    fn create_class(
        &self,
        class: &LiveClass,
    ) -> Result<LiveClass, LiveClassError> {
        let mut conn = self.get_conn()?;

        let new_row = NewLiveClassRow {
            id: class.id,
            teacher_id: class.teacher_id,
            language_id: class.language_id,
            title: &class.title,
            description: class.description.as_deref(),
            scheduled_at: class.scheduled_at,
            duration_minutes: class.duration_minutes,
            max_students: class.max_students,
            status: &class.status,
            meeting_url: class.meeting_url.as_deref(),
        };

        let row = diesel::insert_into(live_classes::table)
            .values(&new_row)
            .returning(LiveClassRow::as_returning())
            .get_result::<LiveClassRow>(&mut conn)
            .map_err(|e| LiveClassError::RepositoryError(e.to_string()))?;

        Ok(row_to_live_class(row))
    }

    fn find_class_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<LiveClass>, LiveClassError> {
        let mut conn = self.get_conn()?;

        let row = live_classes::table
            .filter(live_classes::id.eq(id))
            .select(LiveClassRow::as_select())
            .first::<LiveClassRow>(&mut conn)
            .optional()
            .map_err(|e| LiveClassError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_live_class))
    }

    fn list_classes(&self) -> Result<Vec<LiveClass>, LiveClassError> {
        let mut conn = self.get_conn()?;

        let rows = live_classes::table
            .order(live_classes::scheduled_at.asc())
            .select(LiveClassRow::as_select())
            .load::<LiveClassRow>(&mut conn)
            .map_err(|e| LiveClassError::RepositoryError(e.to_string()))?;

        Ok(rows.into_iter().map(row_to_live_class).collect())
    }

    fn create_enrollment(
        &self,
        enrollment: &ClassEnrollment,
    ) -> Result<ClassEnrollment, LiveClassError> {
        let mut conn = self.get_conn()?;

        let new_row = NewClassEnrollmentRow {
            id: enrollment.id,
            class_id: enrollment.class_id,
            student_id: enrollment.student_id,
            enrolled_at: enrollment.enrolled_at,
            attended: enrollment.attended,
        };

        let row = diesel::insert_into(class_enrollments::table)
            .values(&new_row)
            .returning(ClassEnrollmentRow::as_returning())
            .get_result::<ClassEnrollmentRow>(&mut conn)
            .map_err(|e| LiveClassError::RepositoryError(e.to_string()))?;

        Ok(row_to_enrollment(row))
    }

    fn find_enrollment(
        &self,
        class_id: Uuid,
        student_id: Uuid,
    ) -> Result<Option<ClassEnrollment>, LiveClassError> {
        let mut conn = self.get_conn()?;

        let row = class_enrollments::table
            .filter(
                class_enrollments::class_id
                    .eq(class_id)
                    .and(class_enrollments::student_id.eq(student_id)),
            )
            .select(ClassEnrollmentRow::as_select())
            .first::<ClassEnrollmentRow>(&mut conn)
            .optional()
            .map_err(|e| LiveClassError::RepositoryError(e.to_string()))?;

        Ok(row.map(row_to_enrollment))
    }

    fn count_enrollments(
        &self,
        class_id: Uuid,
    ) -> Result<i64, LiveClassError> {
        let mut conn = self.get_conn()?;

        let count = class_enrollments::table
            .filter(class_enrollments::class_id.eq(class_id))
            .count()
            .get_result::<i64>(&mut conn)
            .map_err(|e| LiveClassError::RepositoryError(e.to_string()))?;

        Ok(count)
    }

    fn find_availability_by_teacher(
        &self,
        teacher_id: Uuid,
    ) -> Result<Vec<TeacherAvailability>, LiveClassError> {
        let mut conn = self.get_conn()?;

        let rows = teacher_availability::table
            .filter(teacher_availability::teacher_id.eq(teacher_id))
            .order(teacher_availability::day_of_week.asc())
            .select(TeacherAvailabilityRow::as_select())
            .load::<TeacherAvailabilityRow>(&mut conn)
            .map_err(|e| LiveClassError::RepositoryError(e.to_string()))?;

        Ok(rows.into_iter().map(row_to_availability).collect())
    }

    fn replace_availability(
        &self,
        teacher_id: Uuid,
        slots: &[TeacherAvailability],
    ) -> Result<Vec<TeacherAvailability>, LiveClassError> {
        let mut conn = self.get_conn()?;

        // Delete existing slots
        diesel::delete(
            teacher_availability::table
                .filter(teacher_availability::teacher_id.eq(teacher_id)),
        )
        .execute(&mut conn)
        .map_err(|e| LiveClassError::RepositoryError(e.to_string()))?;

        // Insert new slots
        let new_rows: Vec<NewTeacherAvailabilityRow> = slots
            .iter()
            .map(|s| NewTeacherAvailabilityRow {
                id: s.id,
                teacher_id: s.teacher_id,
                day_of_week: s.day_of_week,
                start_time: &s.start_time,
                end_time: &s.end_time,
            })
            .collect();

        diesel::insert_into(teacher_availability::table)
            .values(&new_rows)
            .execute(&mut conn)
            .map_err(|e| LiveClassError::RepositoryError(e.to_string()))?;

        self.find_availability_by_teacher(teacher_id)
    }
}

fn row_to_live_class(row: LiveClassRow) -> LiveClass {
    LiveClass {
        id: row.id,
        teacher_id: row.teacher_id,
        language_id: row.language_id,
        title: row.title,
        description: row.description,
        scheduled_at: row.scheduled_at,
        duration_minutes: row.duration_minutes,
        max_students: row.max_students,
        status: row.status,
        meeting_url: row.meeting_url,
    }
}

fn row_to_enrollment(row: ClassEnrollmentRow) -> ClassEnrollment {
    ClassEnrollment {
        id: row.id,
        class_id: row.class_id,
        student_id: row.student_id,
        enrolled_at: row.enrolled_at,
        attended: row.attended,
    }
}

fn row_to_availability(row: TeacherAvailabilityRow) -> TeacherAvailability {
    TeacherAvailability {
        id: row.id,
        teacher_id: row.teacher_id,
        day_of_week: row.day_of_week,
        start_time: row.start_time,
        end_time: row.end_time,
    }
}
