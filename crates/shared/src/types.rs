use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Strongly-typed wrapper for Student IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StudentId(pub Uuid);

/// Strongly-typed wrapper for Teacher IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TeacherId(pub Uuid);

/// Strongly-typed wrapper for Language IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LanguageId(pub Uuid);

/// Strongly-typed wrapper for Curriculum IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CurriculumId(pub Uuid);

/// Strongly-typed wrapper for Module IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModuleId(pub Uuid);

/// Strongly-typed wrapper for Lesson IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LessonId(pub Uuid);

/// Strongly-typed wrapper for Assessment IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssessmentId(pub Uuid);

/// Strongly-typed wrapper for Credit IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CreditId(pub Uuid);

/// Strongly-typed wrapper for School IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SchoolId(pub Uuid);

/// User roles in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Student,
    Parent,
    Teacher,
    SchoolAdmin,
    PlatformAdmin,
}

/// Education board affiliation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EducationBoard {
    Cbse,
    Icse,
    StateBoard(String),
    Nios,
}

/// Skill type for language learning tracks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillType {
    Reading,
    Writing,
    Listening,
    Speaking,
}

/// Proficiency level determined by diagnostic test.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProficiencyLevel {
    Beginner,
    Intermediate,
    Advanced,
}

/// Grade/class level (1-12).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Grade(u8);

impl Grade {
    /// Create a new Grade, returning None if outside 1-12 range.
    pub fn new(value: u8) -> Option<Self> {
        if (1..=12).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}
