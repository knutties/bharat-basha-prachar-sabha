$version: "2.0"

namespace com.bbps.curriculum

use com.bbps.common#Uuid
use com.bbps.common#Timestamp
use com.bbps.common#SkillType
use com.bbps.common#ProficiencyLevel
use com.bbps.common#Grade
use com.bbps.common#PaginationInput
use com.bbps.common#PaginationOutput
use com.bbps.common#ValidationError
use com.bbps.common#NotFoundError
use com.bbps.common#UnauthorizedError
use com.bbps.common#ForbiddenError

/// Curriculum, module, and lesson management service.
@title("BBPS Curriculum Service")
service CurriculumService {
    version: "1.0"
    operations: [
        ListLanguages
        GetCurriculum
        ListModules
        GetModule
        GetLesson
        EnrollInLanguage
        GetLearningPath
        RecordProgress
    ]
}

// ─── List Languages ─────────────────────────────────────────────

@http(method: "GET", uri: "/api/v1/languages")
@readonly
operation ListLanguages {
    input := {}

    output := {
        @required
        languages: LanguageSummaryList
    }
}

list LanguageSummaryList {
    member: LanguageSummary
}

structure LanguageSummary {
    @required
    id: Uuid

    @required
    name: String

    @required
    script: String

    @required
    isoCode: String

    @required
    supportedGrades: GradeList
}

list GradeList {
    member: Grade
}

// ─── Get Curriculum ─────────────────────────────────────────────

@http(method: "GET", uri: "/api/v1/languages/{languageId}/curriculum")
@readonly
operation GetCurriculum {
    input := {
        @required
        @httpLabel
        languageId: Uuid

        @httpQuery("grade")
        grade: Grade
    }

    output := {
        @required
        curriculum: CurriculumDetail
    }

    errors: [NotFoundError]
}

structure CurriculumDetail {
    @required
    id: Uuid

    @required
    languageId: Uuid

    @required
    grade: Grade

    @required
    title: String

    description: String

    @required
    modules: ModuleSummaryList
}

list ModuleSummaryList {
    member: ModuleSummary
}

structure ModuleSummary {
    @required
    id: Uuid

    @required
    title: String

    @required
    skillType: SkillType

    @required
    orderIndex: Integer

    @required
    lessonCount: Integer

    @required
    estimatedMinutes: Integer
}

// ─── List Modules ───────────────────────────────────────────────

@http(method: "GET", uri: "/api/v1/languages/{languageId}/modules")
@readonly
operation ListModules {
    input := {
        @required
        @httpLabel
        languageId: Uuid

        @httpQuery("grade")
        grade: Grade

        @httpQuery("skillType")
        skillType: SkillType

        @httpQuery("page")
        page: Integer

        @httpQuery("pageSize")
        pageSize: Integer
    }

    output := {
        @required
        modules: ModuleSummaryList

        @required
        pagination: PaginationOutput
    }

    errors: [NotFoundError]
}

// ─── Get Module ─────────────────────────────────────────────────

@http(method: "GET", uri: "/api/v1/modules/{moduleId}")
@readonly
operation GetModule {
    input := {
        @required
        @httpLabel
        moduleId: Uuid
    }

    output := {
        @required
        module: ModuleDetail
    }

    errors: [NotFoundError]
}

structure ModuleDetail {
    @required
    id: Uuid

    @required
    title: String

    @required
    description: String

    @required
    skillType: SkillType

    @required
    grade: Grade

    @required
    orderIndex: Integer

    @required
    lessons: LessonSummaryList

    @required
    estimatedMinutes: Integer
}

list LessonSummaryList {
    member: LessonSummary
}

structure LessonSummary {
    @required
    id: Uuid

    @required
    title: String

    @required
    lessonType: LessonType

    @required
    orderIndex: Integer

    @required
    durationMinutes: Integer
}

enum LessonType {
    VIDEO
    TEXT
    INTERACTIVE
    EXERCISE
    CULTURAL
}

// ─── Get Lesson ─────────────────────────────────────────────────

@http(method: "GET", uri: "/api/v1/lessons/{lessonId}")
@readonly
operation GetLesson {
    input := {
        @required
        @httpLabel
        lessonId: Uuid
    }

    output := {
        @required
        lesson: LessonDetail
    }

    errors: [NotFoundError, UnauthorizedError]
}

structure LessonDetail {
    @required
    id: Uuid

    @required
    title: String

    @required
    description: String

    @required
    lessonType: LessonType

    /// Lesson content in structured JSON format.
    @required
    content: String

    mediaUrls: MediaUrlList

    @required
    durationMinutes: Integer
}

list MediaUrlList {
    member: String
}

// ─── Enroll in Language ─────────────────────────────────────────

@http(method: "POST", uri: "/api/v1/enrollments")
operation EnrollInLanguage {
    input := {
        @required
        studentId: Uuid

        @required
        languageId: Uuid

        @required
        grade: Grade

        initialProficiency: ProficiencyLevel
    }

    output := {
        @required
        enrollmentId: Uuid

        @required
        message: String
    }

    errors: [ValidationError, NotFoundError, UnauthorizedError]
}

// ─── Get Learning Path ──────────────────────────────────────────

@http(method: "GET", uri: "/api/v1/enrollments/{enrollmentId}/learning-path")
@readonly
operation GetLearningPath {
    input := {
        @required
        @httpLabel
        enrollmentId: Uuid
    }

    output := {
        @required
        enrollmentId: Uuid

        @required
        languageName: String

        @required
        currentModule: ModuleSummary

        @required
        completedModules: Integer

        @required
        totalModules: Integer

        @required
        overallProgress: Float

        @required
        nextModules: ModuleSummaryList
    }

    errors: [NotFoundError, UnauthorizedError]
}

// ─── Record Progress ────────────────────────────────────────────

@http(method: "POST", uri: "/api/v1/progress")
operation RecordProgress {
    input := {
        @required
        studentId: Uuid

        @required
        lessonId: Uuid

        @required
        completedAt: Timestamp

        /// Score as percentage (0-100), if applicable.
        score: Float

        /// Time spent in seconds.
        @required
        timeSpentSeconds: Integer
    }

    output := {
        @required
        progressId: Uuid

        @required
        message: String

        /// True if this completes the parent module.
        moduleCompleted: Boolean
    }

    errors: [ValidationError, NotFoundError, UnauthorizedError]
}
