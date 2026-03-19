use chrono::Utc;
use shared::types::ProficiencyLevel;
use uuid::Uuid;

use super::errors::CurriculumError;
use super::models::{
    EnrollInput, Enrollment, Language, LearningPath, Lesson, Module,
    Progress, RecordProgressInput,
};
use super::repository::{CurriculumRepository, EnrollmentRepository};

/// Core curriculum and enrollment service.
/// Contains all business logic; depends only on repository traits.
pub struct CurriculumServiceImpl<CR: CurriculumRepository, ER: EnrollmentRepository> {
    curriculum_repo: CR,
    enrollment_repo: ER,
}

impl<CR: CurriculumRepository, ER: EnrollmentRepository>
    CurriculumServiceImpl<CR, ER>
{
    pub fn new(curriculum_repo: CR, enrollment_repo: ER) -> Self {
        Self {
            curriculum_repo,
            enrollment_repo,
        }
    }

    /// List all available languages.
    pub fn list_languages(&self) -> Result<Vec<Language>, CurriculumError> {
        self.curriculum_repo.list_languages()
    }

    /// Get curriculum details for a language, optionally filtered by grade.
    pub fn get_curriculum(
        &self,
        language_id: Uuid,
        grade: Option<shared::types::Grade>,
    ) -> Result<
        (super::models::Curriculum, Vec<Module>),
        CurriculumError,
    > {
        // Verify language exists
        self.curriculum_repo
            .find_language_by_id(language_id)?
            .ok_or_else(|| {
                CurriculumError::LanguageNotFound(language_id.to_string())
            })?;

        let curriculum = self
            .curriculum_repo
            .find_curriculum(language_id, grade)?
            .ok_or_else(|| {
                CurriculumError::CurriculumNotFound(format!(
                    "language_id={language_id}"
                ))
            })?;

        let modules =
            self.curriculum_repo.list_modules(curriculum.id, None)?;

        Ok((curriculum, modules))
    }

    /// List modules for a language, with optional grade and skill type filters.
    pub fn list_modules(
        &self,
        language_id: Uuid,
        grade: Option<shared::types::Grade>,
        skill_type: Option<shared::types::SkillType>,
    ) -> Result<Vec<Module>, CurriculumError> {
        // Verify language exists
        self.curriculum_repo
            .find_language_by_id(language_id)?
            .ok_or_else(|| {
                CurriculumError::LanguageNotFound(language_id.to_string())
            })?;

        let curriculum = self
            .curriculum_repo
            .find_curriculum(language_id, grade)?
            .ok_or_else(|| {
                CurriculumError::CurriculumNotFound(format!(
                    "language_id={language_id}"
                ))
            })?;

        self.curriculum_repo
            .list_modules(curriculum.id, skill_type)
    }

    /// Get a single module with its lessons.
    pub fn get_module(
        &self,
        module_id: Uuid,
    ) -> Result<(Module, Vec<Lesson>), CurriculumError> {
        let module = self
            .curriculum_repo
            .find_module(module_id)?
            .ok_or_else(|| {
                CurriculumError::ModuleNotFound(module_id.to_string())
            })?;

        let lessons =
            self.curriculum_repo.list_lessons_for_module(module_id)?;

        Ok((module, lessons))
    }

    /// Get a single lesson by ID.
    pub fn get_lesson(
        &self,
        lesson_id: Uuid,
    ) -> Result<Lesson, CurriculumError> {
        self.curriculum_repo
            .find_lesson(lesson_id)?
            .ok_or_else(|| {
                CurriculumError::LessonNotFound(lesson_id.to_string())
            })
    }

    /// Enroll a student in a language course.
    pub fn enroll(
        &self,
        input: EnrollInput,
    ) -> Result<Enrollment, CurriculumError> {
        // Verify language exists
        self.curriculum_repo
            .find_language_by_id(input.language_id)?
            .ok_or_else(|| {
                CurriculumError::LanguageNotFound(
                    input.language_id.to_string(),
                )
            })?;

        // Check for duplicate enrollment
        if self
            .enrollment_repo
            .find_enrollment_by_student_and_language(
                input.student_id,
                input.language_id,
            )?
            .is_some()
        {
            return Err(CurriculumError::AlreadyEnrolled(format!(
                "student={} language={}",
                input.student_id, input.language_id
            )));
        }

        let enrollment = Enrollment {
            id: Uuid::new_v4(),
            student_id: input.student_id,
            language_id: input.language_id,
            grade: input.grade,
            proficiency: input
                .initial_proficiency
                .unwrap_or(ProficiencyLevel::Beginner),
            enrolled_at: Utc::now(),
        };

        self.enrollment_repo.create_enrollment(&enrollment)
    }

    /// Get the learning path for an enrollment.
    pub fn get_learning_path(
        &self,
        enrollment_id: Uuid,
    ) -> Result<LearningPath, CurriculumError> {
        let enrollment = self
            .enrollment_repo
            .find_enrollment(enrollment_id)?
            .ok_or_else(|| {
                CurriculumError::EnrollmentNotFound(
                    enrollment_id.to_string(),
                )
            })?;

        let language = self
            .curriculum_repo
            .find_language_by_id(enrollment.language_id)?
            .ok_or_else(|| {
                CurriculumError::LanguageNotFound(
                    enrollment.language_id.to_string(),
                )
            })?;

        let curriculum = self
            .curriculum_repo
            .find_curriculum(
                enrollment.language_id,
                Some(enrollment.grade),
            )?
            .ok_or_else(|| {
                CurriculumError::CurriculumNotFound(format!(
                    "language_id={}",
                    enrollment.language_id
                ))
            })?;

        let modules =
            self.curriculum_repo.list_modules(curriculum.id, None)?;

        let total_modules = modules.len() as i32;

        // Determine completed modules by checking progress
        let mut completed_count = 0i32;
        for module in &modules {
            let lessons = self
                .curriculum_repo
                .list_lessons_for_module(module.id)?;
            let progress = self
                .enrollment_repo
                .get_module_progress(enrollment.student_id, module.id)?;

            let completed_lesson_ids: std::collections::HashSet<Uuid> =
                progress.iter().map(|p| p.lesson_id).collect();
            let all_done = !lessons.is_empty()
                && lessons
                    .iter()
                    .all(|l| completed_lesson_ids.contains(&l.id));
            if all_done {
                completed_count += 1;
            }
        }

        let overall_progress = if total_modules > 0 {
            (completed_count as f64 / total_modules as f64) * 100.0
        } else {
            0.0
        };

        // Current module is the first incomplete one, or the last one.
        let current_module_idx = completed_count.min(total_modules - 1).max(0) as usize;
        let current_module = modules
            .get(current_module_idx)
            .cloned()
            .ok_or_else(|| {
                CurriculumError::ModuleNotFound(
                    "No modules in curriculum".to_string(),
                )
            })?;

        // Next modules are up to 3 modules after the current one.
        let next_start = current_module_idx + 1;
        let next_modules: Vec<Module> = modules
            .into_iter()
            .skip(next_start)
            .take(3)
            .collect();

        Ok(LearningPath {
            enrollment_id,
            language_name: language.name,
            current_module,
            completed_modules: completed_count,
            total_modules,
            overall_progress,
            next_modules,
        })
    }

    /// Record a student's progress on a lesson.
    pub fn record_progress(
        &self,
        input: RecordProgressInput,
    ) -> Result<(Progress, bool), CurriculumError> {
        // Validate lesson exists and get its module
        let lesson = self
            .curriculum_repo
            .find_lesson(input.lesson_id)?
            .ok_or_else(|| {
                CurriculumError::LessonNotFound(
                    input.lesson_id.to_string(),
                )
            })?;

        // Validate score range if provided
        if let Some(score) = input.score {
            if !(0.0..=100.0).contains(&score) {
                return Err(CurriculumError::ValidationError(
                    "Score must be between 0 and 100".to_string(),
                ));
            }
        }

        let progress = Progress {
            id: Uuid::new_v4(),
            student_id: input.student_id,
            lesson_id: input.lesson_id,
            completed_at: input.completed_at,
            score: input.score,
            time_spent_seconds: input.time_spent_seconds,
        };

        let saved = self.enrollment_repo.record_progress(&progress)?;

        // Check if this completes the module
        let all_lessons = self
            .curriculum_repo
            .list_lessons_for_module(lesson.module_id)?;
        let module_progress = self
            .enrollment_repo
            .get_module_progress(input.student_id, lesson.module_id)?;

        let completed_ids: std::collections::HashSet<Uuid> =
            module_progress.iter().map(|p| p.lesson_id).collect();
        let module_completed = !all_lessons.is_empty()
            && all_lessons
                .iter()
                .all(|l| completed_ids.contains(&l.id));

        Ok((saved, module_completed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::LessonType;
    use crate::domain::repository::{
        MockCurriculumRepository, MockEnrollmentRepository,
    };
    use shared::types::{Grade, SkillType};

    fn make_language() -> Language {
        Language {
            id: Uuid::new_v4(),
            name: "Tamil".to_string(),
            script: "Tamil".to_string(),
            iso_code: "ta".to_string(),
            family: "Dravidian".to_string(),
        }
    }

    #[test]
    fn test_list_languages() {
        let mut curriculum_repo = MockCurriculumRepository::new();
        let enrollment_repo = MockEnrollmentRepository::new();

        let lang = make_language();
        let lang_clone = lang.clone();

        curriculum_repo
            .expect_list_languages()
            .returning(move || Ok(vec![lang_clone.clone()]));

        let service =
            CurriculumServiceImpl::new(curriculum_repo, enrollment_repo);

        let result = service.list_languages();
        assert!(result.is_ok());
        let languages = result.unwrap();
        assert_eq!(languages.len(), 1);
        assert_eq!(languages[0].name, "Tamil");
    }

    #[test]
    fn test_enroll_success() {
        let mut curriculum_repo = MockCurriculumRepository::new();
        let mut enrollment_repo = MockEnrollmentRepository::new();

        let lang = make_language();
        let lang_id = lang.id;

        curriculum_repo
            .expect_find_language_by_id()
            .returning(move |_| Ok(Some(lang.clone())));

        enrollment_repo
            .expect_find_enrollment_by_student_and_language()
            .returning(|_, _| Ok(None));

        enrollment_repo
            .expect_create_enrollment()
            .returning(|e| Ok(e.clone()));

        let service =
            CurriculumServiceImpl::new(curriculum_repo, enrollment_repo);

        let input = EnrollInput {
            student_id: Uuid::new_v4(),
            language_id: lang_id,
            grade: Grade::new(7).unwrap(),
            initial_proficiency: None,
        };

        let result = service.enroll(input);
        assert!(result.is_ok());
        let enrollment = result.unwrap();
        assert_eq!(enrollment.proficiency, ProficiencyLevel::Beginner);
    }

    #[test]
    fn test_enroll_already_enrolled() {
        let mut curriculum_repo = MockCurriculumRepository::new();
        let mut enrollment_repo = MockEnrollmentRepository::new();

        let lang = make_language();
        let lang_id = lang.id;
        let student_id = Uuid::new_v4();

        curriculum_repo
            .expect_find_language_by_id()
            .returning(move |_| Ok(Some(lang.clone())));

        let existing = Enrollment {
            id: Uuid::new_v4(),
            student_id,
            language_id: lang_id,
            grade: Grade::new(7).unwrap(),
            proficiency: ProficiencyLevel::Beginner,
            enrolled_at: Utc::now(),
        };
        enrollment_repo
            .expect_find_enrollment_by_student_and_language()
            .returning(move |_, _| Ok(Some(existing.clone())));

        let service =
            CurriculumServiceImpl::new(curriculum_repo, enrollment_repo);

        let input = EnrollInput {
            student_id,
            language_id: lang_id,
            grade: Grade::new(7).unwrap(),
            initial_proficiency: None,
        };

        let result = service.enroll(input);
        assert!(matches!(result, Err(CurriculumError::AlreadyEnrolled(_))));
    }

    #[test]
    fn test_record_progress_validates_score() {
        let mut curriculum_repo = MockCurriculumRepository::new();
        let enrollment_repo = MockEnrollmentRepository::new();

        let lesson = Lesson {
            id: Uuid::new_v4(),
            module_id: Uuid::new_v4(),
            title: "Lesson 1".to_string(),
            description: "Test lesson".to_string(),
            lesson_type: LessonType::Text,
            content: "{}".to_string(),
            media_urls: vec![],
            duration_minutes: 10,
            order_index: 1,
        };
        let lesson_id = lesson.id;

        curriculum_repo
            .expect_find_lesson()
            .returning(move |_| Ok(Some(lesson.clone())));

        let service =
            CurriculumServiceImpl::new(curriculum_repo, enrollment_repo);

        let input = RecordProgressInput {
            student_id: Uuid::new_v4(),
            lesson_id,
            completed_at: Utc::now(),
            score: Some(150.0), // Invalid
            time_spent_seconds: 300,
        };

        let result = service.record_progress(input);
        assert!(matches!(
            result,
            Err(CurriculumError::ValidationError(_))
        ));
    }
}
