# Bharat Basha Prachar Sabha — Project Guide

## Project Overview

A platform enabling Indian school students to learn their mother tongue from anywhere in India and earn board-recognized school credits. See `PLAN.md` for the full product and technical plan.

## Architecture

### Monorepo Structure

```
├── smithy/                    # Smithy IDL service models (source of truth for all APIs)
├── crates/
│   ├── shared/                # Shared Rust types, error handling, utilities
│   ├── auth-service/          # User & authentication service
│   ├── curriculum-service/    # Curriculum, modules, lessons
│   ├── assessment-service/    # Assessments, grading, credit issuance
│   ├── content-service/       # Content management (CMS)
│   ├── live-class-service/    # Live class scheduling & WebRTC signaling
│   ├── notification-service/  # Email, SMS, push notifications
│   └── payment-service/       # Razorpay integration, subscriptions
├── services/
│   └── ai-ml/                 # Python FastAPI service (speech, NLP, OCR)
├── packages/
│   ├── web/                   # React + TypeScript web app
│   └── mobile/                # React Native app (future)
├── infrastructure/            # Docker, K8s, Terraform
├── docs/                      # Architecture decisions, data model, API docs
└── scripts/                   # Dev tooling, seed data, local setup
```

### Three-Layer Service Architecture

Every Rust service follows API / Domain / Database separation:

**API Layer** (`src/api/`)
- Axum route handlers and middleware
- Implements Smithy-generated server traits
- Request validation, response serialization
- Converts between Smithy types and domain types
- NO business logic, NO direct database access

**Domain Layer** (`src/domain/`)
- Pure business logic and rules
- Domain model types (distinct from DB models and API DTOs)
- Defines repository traits (interfaces)
- Zero dependencies on Axum, Diesel, or any infrastructure crate
- Unit-testable with mock repositories

**Database Layer** (`src/db/`)
- Diesel ORM models, schema, and migrations
- Implements repository traits defined in the domain layer
- Maps database rows to/from domain types
- NO business logic, NO HTTP awareness

**Dependency rule:** API → Domain ← Database. Domain never depends outward.

## Technology Stack

- **Language (Backend):** Rust (edition 2021)
- **Web Framework:** Axum 0.7+ with Tokio async runtime
- **ORM:** Diesel 2.x with PostgreSQL backend
- **API Definition:** Smithy IDL
- **Frontend:** React 18 + TypeScript 5 + Tailwind CSS
- **AI/ML:** Python 3.11+ with FastAPI
- **Database:** PostgreSQL 16
- **Cache:** Redis 7
- **Message Queue:** RabbitMQ

## Coding Conventions

### Rust

- Use `thiserror` for error types in domain layer, `anyhow` only in binaries/main
- Prefer `impl Trait` over `dyn Trait` where possible; use `dyn` for repository trait objects
- All public functions must have doc comments
- Use `#[cfg(test)]` modules within each file for unit tests
- Integration tests go in `tests/` directory of each crate
- Run `cargo fmt` before committing (enforced by CI)
- Run `cargo clippy -- -D warnings` — no warnings allowed
- Use `tracing` crate for structured logging (not `println!` or `log`)
- Database IDs use UUIDs (`uuid` crate)
- Timestamps use `chrono::DateTime<Utc>`

### Naming

- Crate names: `kebab-case` (e.g., `auth-service`)
- Module names: `snake_case`
- Types: `PascalCase`
- Functions/methods: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Database tables: `snake_case`, plural (e.g., `students`, `language_enrollments`)
- Database columns: `snake_case`
- Smithy shapes: `PascalCase` for structures, `camelCase` for members

### Error Handling

Each service defines domain errors in `src/domain/errors.rs`:
```rust
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Entity not found: {0}")]
    NotFound(String),
    #[error("Validation failed: {0}")]
    ValidationError(String),
    // ...service-specific variants
}
```
The API layer maps `DomainError` → HTTP status codes. The domain layer never knows about HTTP.

### Testing

- **Unit tests:** Domain layer logic with mock repositories
- **Integration tests:** API layer with test database (use `sqlx` test fixtures or Diesel test transactions)
- **Test naming:** `test_<function_name>_<scenario>` (e.g., `test_create_student_duplicate_email`)
- Minimum test coverage goal: domain layer 80%+

### Git

- Branch naming: `feature/<description>`, `fix/<description>`, `chore/<description>`
- Commit messages: imperative mood, concise first line, body for context
- One logical change per commit

## Development Commands

This project uses [just](https://github.com/casey/just) as the command runner. Multi-level justfiles exist at the root and in each service/package directory.

### Workspace-level (from repo root)

```bash
just                    # List all available recipes
just build              # Build all Rust services
just test               # Run all tests
just fmt                # Format all Rust code
just lint               # Clippy lints (warnings = errors)
just check              # Fast compile check (no codegen)
just precommit          # fmt-check + lint + check
just ci                 # Full CI pipeline locally

# Run/build/test a specific service by short name
just run auth           # cargo run -p auth-service
just test-service auth  # cargo test -p auth-service
just build-service auth # cargo build -p auth-service
just migrate auth       # diesel migration run for auth-service

just migrate-all        # Run migrations for ALL services

# Web app
just web-dev            # Start React dev server
just web-build          # Production build
just web-lint           # Lint frontend code

# AI/ML service
just ai-dev             # Start FastAPI dev server

# Docker
just infra-up           # Start PostgreSQL + Redis
just infra-down         # Stop infrastructure
just docker-up          # Build and start all services
just docker-logs auth-service  # Tail logs for a service

# Smithy
just smithy-validate    # Validate .smithy model files
```

### Service-level (from crates/<service>/)

```bash
just                    # List service-specific recipes
just run                # Run this service
just test               # Test this service
just test-unit          # Domain layer unit tests only
just test-integration   # Integration tests only
just lint               # Clippy for this service
just migrate            # Run database migrations
just migrate-rollback   # Revert last migration
just db-reset           # Drop + create + migrate
just db-schema          # Regenerate Diesel schema from DB
```

### Web app (from packages/web/)

```bash
just install            # npm ci
just dev                # Start dev server
just build              # Production build
just lint               # ESLint
just typecheck          # TypeScript type check
just ci                 # Full frontend CI
```

## Environment Variables

Each service reads from `.env` (local) or environment:

```
DATABASE_URL=postgres://user:pass@localhost:5432/bbps_<service>
REDIS_URL=redis://localhost:6379
RUST_LOG=info,tower_http=debug
PORT=<service-specific>
JWT_SECRET=<shared secret for auth tokens>
```

Service ports:
- auth-service: 8001
- curriculum-service: 8002
- assessment-service: 8003
- content-service: 8004
- live-class-service: 8005
- notification-service: 8006
- payment-service: 8007
- ai-ml-service: 8010
- web app (dev): 3000
