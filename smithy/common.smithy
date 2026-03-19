$version: "2.0"

namespace com.bbps.common

/// UUID-based identifier.
@pattern("^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$")
string Uuid

/// ISO 8601 timestamp.
string Timestamp

/// Email address.
@pattern("^[^@]+@[^@]+\\.[^@]+$")
string Email

/// Indian mobile number.
@pattern("^\\+91[0-9]{10}$")
string PhoneNumber

/// User roles in the system.
enum UserRole {
    STUDENT
    PARENT
    TEACHER
    SCHOOL_ADMIN
    PLATFORM_ADMIN
}

/// Supported education boards.
enum EducationBoard {
    CBSE
    ICSE
    NIOS
    STATE_BOARD
}

/// Language learning skill tracks.
enum SkillType {
    READING
    WRITING
    LISTENING
    SPEAKING
}

/// Student proficiency level.
enum ProficiencyLevel {
    BEGINNER
    INTERMEDIATE
    ADVANCED
}

/// Grade/class level (1-12).
@range(min: 1, max: 12)
integer Grade

/// Pagination input parameters.
structure PaginationInput {
    @range(min: 1)
    page: Integer = 1

    @range(min: 1, max: 100)
    pageSize: Integer = 20
}

/// Pagination metadata in responses.
structure PaginationOutput {
    @required
    page: Integer

    @required
    pageSize: Integer

    @required
    totalItems: Long

    @required
    totalPages: Long
}

/// Standard error structure.
@error("client")
structure ValidationError {
    @required
    message: String

    fieldErrors: FieldErrors
}

list FieldErrors {
    member: FieldError
}

structure FieldError {
    @required
    field: String

    @required
    message: String
}

@error("client")
@httpError(404)
structure NotFoundError {
    @required
    message: String
}

@error("client")
@httpError(401)
structure UnauthorizedError {
    @required
    message: String
}

@error("client")
@httpError(403)
structure ForbiddenError {
    @required
    message: String
}

@error("client")
@httpError(409)
structure ConflictError {
    @required
    message: String
}
