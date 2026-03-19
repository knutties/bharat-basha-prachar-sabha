use anyhow::Result;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// Create a connection pool for the given database URL.
pub fn establish_pool(database_url: &str) -> Result<DbPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().max_size(10).build(manager)?;
    Ok(pool)
}

/// Run pending database migrations.
pub fn run_migrations(pool: &DbPool) -> Result<()> {
    let mut conn = pool.get()?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Migration failed: {}", e))?;
    tracing::info!("Database migrations completed");
    Ok(())
}
