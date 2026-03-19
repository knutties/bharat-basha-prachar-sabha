mod api;
mod db;
mod domain;

use anyhow::Result;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::router::create_router;
use crate::db::connection::establish_pool;
use crate::db::repository::PgLiveClassRepository;
use crate::domain::services::LiveClassServiceImpl;

/// Application state shared across all request handlers.
pub struct AppState {
    pub live_class_service: LiveClassServiceImpl<PgLiveClassRepository>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "live_class_service=debug,tower_http=debug".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = establish_pool(&database_url)?;

    db::connection::run_migrations(&pool)?;

    let repo = PgLiveClassRepository::new(pool);
    let live_class_service = LiveClassServiceImpl::new(repo);

    let state = Arc::new(AppState {
        live_class_service,
    });

    let app = create_router(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8005".to_string());
    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    tracing::info!("Live class service listening on port {port}");
    axum::serve(listener, app).await?;

    Ok(())
}
