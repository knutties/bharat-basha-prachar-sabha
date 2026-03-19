$version: "2.0"

namespace com.bbps.assessment

use com.bbps.common#Uuid
use com.bbps.common#Timestamp
use com.bbps.common#Grade
use com.bbps.common#PaginationOutput
use com.bbps.common#ValidationError
use com.bbps.common#NotFoundError
use com.bbps.common#UnauthorizedError
use com.bbps.common#ForbiddenError

/// Assessment, grading, and credit issuance service.
@title("BBPS Assessment Service")
service AssessmentService {
    version: "1.0"
    operations: [
        CreateAssessment
        SubmitAssessment
        GetAssessmentResult
        ListStudentCredits
        VerifyCredential
    ]
}

enum AssessmentType {
    MODULE_QUIZ
    SEMESTER_EXAM
    ORAL_EXAM
    DIAGNOSTIC
}

enum AssessmentStatus {
    NOT_STARTED
    IN_PROGRESS
    SUBMITTED
    GRADED
}

enum CreditStatus {
    PENDING
    ISSUED
    VERIFIED
    REVOKED
}

// ─── Create Assessment ──────────────────────────────────────────

@http(method: "POST", uri: "/api/v1/assessments")
operation CreateAssessment {
    input := {
        @required
        studentId: Uuid

        @required
        moduleId: Uuid

        @required
        assessmentType: AssessmentType
    }

    output := {
        @required
        assessmentId: Uuid

        @required
        questions: QuestionList

        @required
        timeLimitMinutes: Integer

        @required
        startedAt: Timestamp
    }

    errors: [ValidationError, NotFoundError, UnauthorizedError]
}

list QuestionList {
    member: Question
}

structure Question {
    @required
    id: Uuid

    @required
    questionText: String

    @required
    questionType: QuestionType

    /// Available options for multiple-choice questions.
    options: OptionList

    /// Media URL for audio/image-based questions.
    mediaUrl: String

    @required
    maxScore: Integer
}

enum QuestionType {
    MULTIPLE_CHOICE
    FILL_IN_BLANK
    SHORT_ANSWER
    AUDIO_RESPONSE
    MATCH_PAIRS
}

list OptionList {
    member: String
}

// ─── Submit Assessment ──────────────────────────────────────────

@http(method: "POST", uri: "/api/v1/assessments/{assessmentId}/submit")
operation SubmitAssessment {
    input := {
        @required
        @httpLabel
        assessmentId: Uuid

        @required
        answers: AnswerList
    }

    output := {
        @required
        assessmentId: Uuid

        @required
        status: AssessmentStatus

        /// Score (available immediately for auto-graded; null for oral exams).
        score: Float

        maxScore: Float

        passed: Boolean

        @required
        submittedAt: Timestamp
    }

    errors: [ValidationError, NotFoundError, UnauthorizedError]
}

list AnswerList {
    member: Answer
}

structure Answer {
    @required
    questionId: Uuid

    @required
    answer: String
}

// ─── Get Assessment Result ──────────────────────────────────────

@http(method: "GET", uri: "/api/v1/assessments/{assessmentId}/result")
@readonly
operation GetAssessmentResult {
    input := {
        @required
        @httpLabel
        assessmentId: Uuid
    }

    output := {
        @required
        assessmentId: Uuid

        @required
        status: AssessmentStatus

        @required
        score: Float

        @required
        maxScore: Float

        @required
        passed: Boolean

        @required
        gradeLetter: String

        feedback: String

        creditId: Uuid
    }

    errors: [NotFoundError, UnauthorizedError]
}

// ─── List Student Credits ───────────────────────────────────────

@http(method: "GET", uri: "/api/v1/students/{studentId}/credits")
@readonly
operation ListStudentCredits {
    input := {
        @required
        @httpLabel
        studentId: Uuid

        @httpQuery("page")
        page: Integer

        @httpQuery("pageSize")
        pageSize: Integer
    }

    output := {
        @required
        credits: CreditList

        @required
        pagination: PaginationOutput
    }

    errors: [NotFoundError, UnauthorizedError]
}

list CreditList {
    member: Credit
}

structure Credit {
    @required
    id: Uuid

    @required
    studentId: Uuid

    @required
    languageName: String

    @required
    grade: Grade

    @required
    semester: String

    @required
    score: Float

    @required
    gradeLetter: String

    @required
    creditPoints: Float

    @required
    status: CreditStatus

    @required
    certificateUrl: String

    @required
    verificationCode: String

    @required
    issuedAt: Timestamp
}

// ─── Verify Credential ─────────────────────────────────────────

@http(method: "GET", uri: "/api/v1/credentials/verify/{verificationCode}")
@readonly
operation VerifyCredential {
    input := {
        @required
        @httpLabel
        verificationCode: String
    }

    output := {
        @required
        valid: Boolean

        @required
        studentName: String

        @required
        languageName: String

        @required
        grade: Grade

        @required
        gradeLetter: String

        @required
        creditPoints: Float

        @required
        issuedAt: Timestamp
    }

    errors: [NotFoundError]
}
