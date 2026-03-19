//! Integration tests for the auth-service API layer.
//!
//! These tests start a real Axum app backed by a test PostgreSQL database
//! and exercise the full request → handler → domain → DB → response pipeline.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;
use tower::util::ServiceExt;

use auth_service::api::router::create_router;
use auth_service::db::connection::run_migrations;
use auth_service::db::repository::PgUserRepository;
use auth_service::domain::services::AuthServiceImpl;
use auth_service::AppState;

type DbPool = Pool<ConnectionManager<PgConnection>>;

fn test_database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://bbps:bbps_dev_password@localhost:5432/bbps_auth".to_string()
    })
}

fn create_pool(url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .max_size(2)
        .build(manager)
        .expect("Failed to create test pool")
}

fn setup_app() -> axum::Router {
    let pool = create_pool(&test_database_url());

    run_migrations(&pool).expect("migrations failed");

    // Clean tables for test isolation
    {
        let mut conn = pool.get().expect("get connection");
        diesel::sql_query(
            "TRUNCATE teacher_profiles, student_profiles, users CASCADE",
        )
        .execute(&mut conn)
        .expect("truncate");
    }

    let user_repo = PgUserRepository::new(pool);
    let jwt_secret = "test-jwt-secret".to_string();
    let auth_service = AuthServiceImpl::new(user_repo, jwt_secret.clone());

    let state = Arc::new(AppState {
        auth_service,
        jwt_secret,
    });

    create_router(state)
}

fn json_request(
    method: &str,
    uri: &str,
    body: serde_json::Value,
) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap()
}

async fn body_json(response: axum::response::Response) -> serde_json::Value {
    let body =
        axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
    serde_json::from_slice(&body).unwrap()
}

// ─── Tests ──────────────────────────────────────────────────────

#[tokio::test]
async fn test_health_check() {
    let app = setup_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json = body_json(response).await;
    assert_eq!(json["status"], "healthy");
    assert_eq!(json["service"], "auth-service");
}

#[tokio::test]
async fn test_register_student_success() {
    let app = setup_app();

    let response = app
        .oneshot(json_request(
            "POST",
            "/api/v1/auth/register/student",
            serde_json::json!({
                "name": "Arjun Kumar",
                "email": "arjun@test.com",
                "password": "securepassword123",
                "phone": "+919876543210",
                "grade": 7,
                "mother_tongue": "Tamil",
                "state_of_residence": "Karnataka"
            }),
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let json = body_json(response).await;
    assert!(json["user_id"].is_string());
    assert!(json["access_token"].is_string());
    assert!(json["refresh_token"].is_string());
    assert!(!json["access_token"].as_str().unwrap().is_empty());
}

#[tokio::test]
async fn test_register_student_with_board() {
    let app = setup_app();

    let response = app
        .oneshot(json_request(
            "POST",
            "/api/v1/auth/register/student",
            serde_json::json!({
                "name": "Priya Verma",
                "email": "priya@test.com",
                "password": "securepassword123",
                "phone": "+919876543210",
                "grade": 10,
                "mother_tongue": "Hindi",
                "state_of_residence": "Maharashtra",
                "board": "CBSE"
            }),
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_register_student_invalid_grade() {
    let app = setup_app();

    let response = app
        .oneshot(json_request(
            "POST",
            "/api/v1/auth/register/student",
            serde_json::json!({
                "name": "Bad Grade",
                "email": "bad@test.com",
                "password": "password123",
                "phone": "+919876543210",
                "grade": 15,
                "mother_tongue": "Hindi",
                "state_of_residence": "Delhi"
            }),
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let json = body_json(response).await;
    assert_eq!(json["error"]["code"], "VALIDATION_ERROR");
}

#[tokio::test]
async fn test_register_student_duplicate_email() {
    let app = setup_app();

    let body = serde_json::json!({
        "name": "First User",
        "email": "duplicate@test.com",
        "password": "password123",
        "phone": "+919876543210",
        "grade": 5,
        "mother_tongue": "Bengali",
        "state_of_residence": "Maharashtra"
    });

    // First registration
    let response = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/v1/auth/register/student",
            body.clone(),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Duplicate registration
    let response = app
        .oneshot(json_request(
            "POST",
            "/api/v1/auth/register/student",
            body,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_login_success() {
    let app = setup_app();

    // Register
    let response = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/v1/auth/register/student",
            serde_json::json!({
                "name": "Login Test",
                "email": "login@test.com",
                "password": "mypassword123",
                "phone": "+919876543210",
                "grade": 8,
                "mother_tongue": "Kannada",
                "state_of_residence": "Tamil Nadu"
            }),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // Login
    let response = app
        .oneshot(json_request(
            "POST",
            "/api/v1/auth/login",
            serde_json::json!({
                "email": "login@test.com",
                "password": "mypassword123"
            }),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json = body_json(response).await;
    assert!(json["user_id"].is_string());
    assert_eq!(json["role"], "Student");
    assert!(json["access_token"].is_string());
}

#[tokio::test]
async fn test_login_wrong_password() {
    let app = setup_app();

    // Register
    let _ = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/v1/auth/register/student",
            serde_json::json!({
                "name": "Wrong Pass",
                "email": "wrongpass@test.com",
                "password": "correct_password",
                "phone": "+919876543210",
                "grade": 6,
                "mother_tongue": "Telugu",
                "state_of_residence": "Andhra Pradesh"
            }),
        ))
        .await
        .unwrap();

    // Login with wrong password
    let response = app
        .oneshot(json_request(
            "POST",
            "/api/v1/auth/login",
            serde_json::json!({
                "email": "wrongpass@test.com",
                "password": "wrong_password"
            }),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_login_nonexistent_user() {
    let app = setup_app();

    let response = app
        .oneshot(json_request(
            "POST",
            "/api/v1/auth/login",
            serde_json::json!({
                "email": "nobody@test.com",
                "password": "whatever"
            }),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_register_teacher_success() {
    let app = setup_app();

    let response = app
        .oneshot(json_request(
            "POST",
            "/api/v1/auth/register/teacher",
            serde_json::json!({
                "name": "Priya Sharma",
                "email": "teacher@test.com",
                "password": "teacherpass123",
                "phone": "+919876543210",
                "languages": ["Tamil", "Hindi"],
                "qualifications": "M.A. Tamil Literature, B.Ed.",
                "bio": "10 years of teaching experience"
            }),
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let json = body_json(response).await;
    assert!(json["user_id"].is_string());
    assert!(json["access_token"].is_string());
}

#[tokio::test]
async fn test_get_profile_with_auth() {
    let app = setup_app();

    // Register and get token
    let response = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/v1/auth/register/student",
            serde_json::json!({
                "name": "Profile Test",
                "email": "profile@test.com",
                "password": "password123",
                "phone": "+919876543210",
                "grade": 10,
                "mother_tongue": "Marathi",
                "state_of_residence": "Gujarat"
            }),
        ))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let reg_json = body_json(response).await;
    let user_id = reg_json["user_id"].as_str().unwrap();
    let token = reg_json["access_token"].as_str().unwrap();

    // Get profile with valid JWT
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/auth/profile/{user_id}"))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json = body_json(response).await;
    assert_eq!(json["name"], "Profile Test");
    assert_eq!(json["email"], "profile@test.com");
    assert_eq!(json["role"], "Student");
}

#[tokio::test]
async fn test_get_profile_without_auth() {
    let app = setup_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri(
                    "/api/v1/auth/profile/00000000-0000-0000-0000-000000000000",
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
