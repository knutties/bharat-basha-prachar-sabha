mod api;
mod db;
mod domain;

use anyhow::Result;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::router::create_router;
use crate::db::connection::establish_pool;
use crate::db::repository::{PgCurriculumRepository, PgEnrollmentRepository};
use crate::domain::services::CurriculumServiceImpl;

/// Application state shared across all request handlers.
pub struct AppState {
    pub curriculum_service:
        CurriculumServiceImpl<PgCurriculumRepository, PgEnrollmentRepository>,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "curriculum_service=debug,tower_http=debug".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database connection pool
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = establish_pool(&database_url)?;

    // Run migrations
    db::connection::run_migrations(&pool)?;

    // Build application state
    let curriculum_repo = PgCurriculumRepository::new(pool.clone());
    let enrollment_repo = PgEnrollmentRepository::new(pool);
    let jwt_secret =
        std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let curriculum_service =
        CurriculumServiceImpl::new(curriculum_repo, enrollment_repo);

    let state = Arc::new(AppState {
        curriculum_service,
        jwt_secret,
    });

    // Build router
    let app = create_router(state);

    // Start server
    let port = std::env::var("PORT").unwrap_or_else(|_| "8002".to_string());
    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    tracing::info!("Curriculum service listening on port {port}");
    axum::serve(listener, app).await?;

    Ok(())
}
