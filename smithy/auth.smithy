$version: "2.0"

namespace com.bbps.auth

use com.bbps.common#Uuid
use com.bbps.common#Timestamp
use com.bbps.common#Email
use com.bbps.common#PhoneNumber
use com.bbps.common#UserRole
use com.bbps.common#Grade
use com.bbps.common#EducationBoard
use com.bbps.common#ValidationError
use com.bbps.common#NotFoundError
use com.bbps.common#UnauthorizedError
use com.bbps.common#ConflictError

/// Authentication and user management service.
@title("BBPS Auth Service")
service AuthService {
    version: "1.0"
    operations: [
        RegisterStudent
        RegisterTeacher
        Login
        RefreshToken
        GetProfile
        UpdateProfile
    ]
}

// ─── Register Student ───────────────────────────────────────────

@http(method: "POST", uri: "/api/v1/auth/register/student")
operation RegisterStudent {
    input := {
        @required
        name: String

        @required
        email: Email

        @required
        password: String

        @required
        phone: PhoneNumber

        @required
        grade: Grade

        @required
        motherTongue: String

        @required
        stateOfResidence: String

        board: EducationBoard

        schoolId: Uuid

        parentEmail: Email
    }

    output := {
        @required
        userId: Uuid

        @required
        accessToken: String

        @required
        refreshToken: String
    }

    errors: [ValidationError, ConflictError]
}

// ─── Register Teacher ───────────────────────────────────────────

@http(method: "POST", uri: "/api/v1/auth/register/teacher")
operation RegisterTeacher {
    input := {
        @required
        name: String

        @required
        email: Email

        @required
        password: String

        @required
        phone: PhoneNumber

        @required
        languages: LanguageList

        @required
        qualifications: String

        bio: String
    }

    output := {
        @required
        userId: Uuid

        @required
        accessToken: String

        @required
        refreshToken: String
    }

    errors: [ValidationError, ConflictError]
}

list LanguageList {
    member: String
}

// ─── Login ──────────────────────────────────────────────────────

@http(method: "POST", uri: "/api/v1/auth/login")
operation Login {
    input := {
        @required
        email: Email

        @required
        password: String
    }

    output := {
        @required
        userId: Uuid

        @required
        role: UserRole

        @required
        accessToken: String

        @required
        refreshToken: String
    }

    errors: [UnauthorizedError]
}

// ─── Refresh Token ──────────────────────────────────────────────

@http(method: "POST", uri: "/api/v1/auth/refresh")
operation RefreshToken {
    input := {
        @required
        refreshToken: String
    }

    output := {
        @required
        accessToken: String

        @required
        refreshToken: String
    }

    errors: [UnauthorizedError]
}

// ─── Get Profile ────────────────────────────────────────────────

@http(method: "GET", uri: "/api/v1/auth/profile/{userId}")
@readonly
operation GetProfile {
    input := {
        @required
        @httpLabel
        userId: Uuid
    }

    output := {
        @required
        userId: Uuid

        @required
        name: String

        @required
        email: Email

        @required
        role: UserRole

        phone: PhoneNumber

        grade: Grade

        motherTongue: String

        stateOfResidence: String

        board: EducationBoard

        @required
        createdAt: Timestamp
    }

    errors: [NotFoundError, UnauthorizedError]
}

// ─── Update Profile ─────────────────────────────────────────────

@http(method: "PUT", uri: "/api/v1/auth/profile/{userId}")
@idempotent
operation UpdateProfile {
    input := {
        @required
        @httpLabel
        userId: Uuid

        name: String

        phone: PhoneNumber

        grade: Grade

        stateOfResidence: String

        board: EducationBoard
    }

    output := {
        @required
        message: String
    }

    errors: [ValidationError, NotFoundError, UnauthorizedError]
}
