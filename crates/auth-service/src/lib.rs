pub mod api;
pub mod db;
pub mod domain;

use db::repository::PgUserRepository;
use domain::services::AuthServiceImpl;

/// Application state shared across all request handlers.
pub struct AppState {
    pub auth_service: AuthServiceImpl<PgUserRepository>,
    pub jwt_secret: String,
}
