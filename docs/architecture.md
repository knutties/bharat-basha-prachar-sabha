# Architecture Decision Records

## ADR-001: Rust for Backend Services

**Status:** Accepted
**Date:** 2026-03-19

**Context:** We need a backend language for 7+ microservices serving potentially millions of students across India. The platform handles sensitive student data, financial transactions, and real-time features.

**Decision:** Use Rust with Axum web framework and Tokio async runtime for all backend services (except AI/ML which stays in Python).

**Rationale:**
- Memory safety without garbage collection — eliminates entire classes of bugs
- High performance — comparable to C/C++, crucial for low-latency API responses across India's varied network conditions
- Fearless concurrency — safe concurrent programming for real-time features (live classes, notifications)
- Strong type system — catches errors at compile time, especially valuable with Diesel's compile-time query checking
- Growing ecosystem for web services (Axum, Tower, Diesel)

**Trade-offs:**
- Steeper learning curve than Node.js or Go
- Longer compile times
- Smaller talent pool for hiring

---

## ADR-002: Smithy IDL for API Contracts

**Status:** Accepted
**Date:** 2026-03-19

**Context:** With 7 microservices and multiple client apps (web, mobile, school portal), we need a single source of truth for API contracts that can generate both server and client code.

**Decision:** Use Smithy IDL to define all service APIs. Generate Rust server traits and TypeScript client SDKs from the models.

**Rationale:**
- Protocol-agnostic — models describe API shape, not HTTP specifics
- Rich constraint system (validation, pagination traits, error shapes) built into the model
- Code generation pipeline produces typed server stubs and client SDKs
- Composable — shared shapes (pagination, errors, common types) are defined once and referenced everywhere
- Better suited than OpenAPI for service-to-service contracts

**Trade-offs:**
- Less industry adoption than OpenAPI (smaller community, fewer tools)
- Requires custom code generation setup for Rust (smithy-rs)
- Team needs to learn Smithy syntax

---

## ADR-003: Three-Layer Service Architecture

**Status:** Accepted
**Date:** 2026-03-19

**Context:** Each microservice needs a clear internal structure that enforces separation of concerns and enables testing.

**Decision:** Every Rust service follows a strict three-layer architecture:
1. **API Layer** — Axum handlers, request/response types, error mapping
2. **Domain Layer** — Pure business logic, domain models, repository traits
3. **Database Layer** — Diesel ORM, schema, repository implementations

**Key rule:** Dependencies point inward. API → Domain ← Database. The domain layer has zero dependencies on Axum, Diesel, or any infrastructure framework.

**Rationale:**
- Domain logic is unit-testable with mock repositories (no database needed)
- Infrastructure can be swapped without touching business logic
- Clear ownership — API team owns HTTP concerns, domain team owns rules, DB team owns queries
- Smithy types stay at the API boundary; domain types are independent

**Trade-offs:**
- More boilerplate (conversion between API types, domain types, and DB models)
- Three sets of types per entity (DTO, domain model, DB row)
- Overkill for very simple CRUD services (but consistency matters more at scale)

---

## ADR-004: PostgreSQL as Single Database Engine

**Status:** Accepted
**Date:** 2026-03-19

**Context:** The original plan included MongoDB for flexible content storage alongside PostgreSQL. We need to choose a data strategy.

**Decision:** Use PostgreSQL exclusively for all relational data, leveraging Diesel ORM for compile-time query safety. Each microservice gets its own database (database-per-service pattern).

**Rationale:**
- Diesel provides compile-time SQL validation — catches query errors before runtime
- PostgreSQL's JSONB columns handle flexible content structures (lesson content, exercise data)
- Single database engine reduces operational complexity
- PostgreSQL's full-text search and GIN indexes handle multilingual content search
- Array columns (used for teacher languages) are natively supported

**Trade-offs:**
- Less flexibility than a document database for deeply nested content
- Need to use JSONB for truly schemaless data (with some loss of type safety)
- Each service having its own DB increases infrastructure cost slightly

---

## ADR-005: Database Per Service

**Status:** Accepted
**Date:** 2026-03-19

**Context:** Multiple microservices need data persistence. We need to decide between shared database vs. database-per-service.

**Decision:** Each service gets its own PostgreSQL database (e.g., bbps_auth, bbps_curriculum, bbps_assessment).

**Rationale:**
- Services are independently deployable and scalable
- No coupling through shared tables
- Each service owns its schema and migrations
- A failure in one database doesn't cascade to others

**Trade-offs:**
- Cross-service queries require API calls (no joins across services)
- Data consistency across services requires eventual consistency patterns
- More databases to manage and back up

---

## ADR-006: JWT-Based Authentication

**Status:** Accepted
**Date:** 2026-03-19

**Context:** We need authentication across multiple services. Students, parents, teachers, and admins all interact with the platform.

**Decision:** Use JWT (JSON Web Tokens) issued by the auth service. Short-lived access tokens (1 hour) + long-lived refresh tokens (30 days). Each service validates tokens independently using a shared secret.

**Rationale:**
- Stateless — no session store needed; each service validates independently
- Contains role information (student/teacher/admin) for authorization
- Well-supported in both Rust (jsonwebtoken crate) and TypeScript
- Refresh tokens handle token rotation without re-authentication

**Trade-offs:**
- Cannot revoke individual access tokens (must wait for expiry)
- Shared JWT secret must be securely distributed to all services
- Token size grows with claims
