# Bharat Basha Prachar Sabha — Root Justfile
# Run `just` to see all available recipes.

set dotenv-load := true

# Default: list all recipes
default:
    @just --list

# ─── Workspace-wide ─────────────────────────────────────────────

# Build all Rust services
build:
    cargo build --workspace

# Build in release mode
build-release:
    cargo build --workspace --release

# Run all tests across the workspace
test:
    cargo test --workspace

# Run tests with output shown
test-verbose:
    cargo test --workspace -- --nocapture

# Format all Rust code
fmt:
    cargo fmt --all

# Check formatting without modifying
fmt-check:
    cargo fmt --all -- --check

# Run clippy lints on all crates
lint:
    cargo clippy --workspace -- -D warnings

# Check all crates compile (fast, no codegen)
check:
    cargo check --workspace

# Format + lint + check (pre-commit gate)
precommit: fmt-check lint check

# Clean build artifacts
clean:
    cargo clean

# ─── Individual Services ────────────────────────────────────────

# Run a specific service (e.g., just run auth)
run service:
    just crates/{{service}}-service/run

# Build a specific service
build-service service:
    just crates/{{service}}-service/build

# Test a specific service
test-service service:
    just crates/{{service}}-service/test

# Run migrations for a specific service
migrate service:
    just crates/{{service}}-service/migrate

# Rollback migrations for a specific service
migrate-rollback service:
    just crates/{{service}}-service/migrate-rollback

# ─── Web App ────────────────────────────────────────────────────

# Run web app dev server
web-dev:
    just packages/web/dev

# Build web app for production
web-build:
    just packages/web/build

# Lint web app
web-lint:
    just packages/web/lint

# Type-check web app
web-typecheck:
    just packages/web/typecheck

# ─── AI/ML Service ──────────────────────────────────────────────

# Run AI/ML service
ai-dev:
    just services/ai-ml/dev

# ─── Docker ─────────────────────────────────────────────────────

# Start all infrastructure (PostgreSQL, Redis)
infra-up:
    docker compose up -d postgres redis

# Stop all infrastructure
infra-down:
    docker compose down

# Start all services via Docker Compose
docker-up:
    docker compose up -d --build

# Stop all Docker services
docker-down:
    docker compose down

# View logs for a specific service
docker-logs service:
    docker compose logs -f {{service}}

# ─── Database ───────────────────────────────────────────────────

# Run migrations for ALL services
migrate-all:
    #!/usr/bin/env bash
    set -euo pipefail
    for svc in auth curriculum assessment content live-class notification payment; do
        echo "=== Migrating $svc-service ==="
        just crates/$svc-service/migrate
    done

# Create databases for local development
db-create:
    psql -U bbps -h localhost -f scripts/init-databases.sql

# ─── Smithy ─────────────────────────────────────────────────────

# Validate Smithy models
smithy-validate:
    #!/usr/bin/env bash
    set -euo pipefail
    for f in smithy/*.smithy; do
        echo "Validating $f..."
        grep -q 'namespace' "$f" || (echo "ERROR: $f missing namespace" && exit 1)
    done
    echo "All Smithy models valid"

# ─── CI ─────────────────────────────────────────────────────────

# Run full CI pipeline locally
ci: fmt-check lint check test
    @echo "CI passed!"
