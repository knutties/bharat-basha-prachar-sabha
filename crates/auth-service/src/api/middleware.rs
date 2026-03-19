use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use super::handlers::{ErrorBody, ErrorResponse};

/// JWT claims extracted from authorization header.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

/// Middleware that validates JWT tokens from the Authorization header.
/// Injects `Claims` as a request extension for downstream handlers.
pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> impl IntoResponse {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_default();

    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: ErrorBody {
                        code: "UNAUTHORIZED".to_string(),
                        message: "Missing or invalid Authorization header"
                            .to_string(),
                    },
                }),
            )
                .into_response();
        }
    };

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(token_data) => {
            request.extensions_mut().insert(token_data.claims);
            next.run(request).await.into_response()
        }
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: ErrorBody {
                    code: "INVALID_TOKEN".to_string(),
                    message: "Invalid or expired token".to_string(),
                },
            }),
        )
            .into_response(),
    }
}
