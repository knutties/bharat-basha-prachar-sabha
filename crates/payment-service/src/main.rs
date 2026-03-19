mod api;
mod db;
mod domain;

use anyhow::Result;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::router::create_router;
use crate::db::connection::establish_pool;
use crate::db::repository::PgPaymentRepository;
use crate::domain::services::PaymentServiceImpl;

/// Application state shared across all request handlers.
pub struct AppState {
    pub payment_service: PaymentServiceImpl<PgPaymentRepository>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "payment_service=debug,tower_http=debug".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = establish_pool(&database_url)?;

    db::connection::run_migrations(&pool)?;

    let repo = PgPaymentRepository::new(pool);
    let payment_service = PaymentServiceImpl::new(repo);

    let state = Arc::new(AppState {
        payment_service,
    });

    let app = create_router(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8007".to_string());
    let listener =
        tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    tracing::info!("Payment service listening on port {port}");
    axum::serve(listener, app).await?;

    Ok(())
}
