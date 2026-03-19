use chrono::Utc;
use shared::types::UserRole;
use uuid::Uuid;

use super::errors::AuthError;
use super::models::{
    RegisterStudentInput, RegisterTeacherInput, StudentProfile,
    TeacherProfile, TokenPair, UpdateProfileInput, User,
};
use super::repository::UserRepository;

/// Core authentication and user management service.
/// Contains all business logic; depends only on the repository trait.
pub struct AuthServiceImpl<R: UserRepository> {
    repo: R,
    jwt_secret: String,
}

impl<R: UserRepository> AuthServiceImpl<R> {
    pub fn new(repo: R, jwt_secret: String) -> Self {
        Self { repo, jwt_secret }
    }

    /// Register a new student account.
    pub fn register_student(
        &self,
        input: RegisterStudentInput,
    ) -> Result<(Uuid, TokenPair), AuthError> {
        // Check for existing email
        if self.repo.find_by_email(&input.email)?.is_some() {
            return Err(AuthError::EmailAlreadyExists(input.email));
        }

        // Hash password
        let password_hash = self.hash_password(&input.password)?;

        // Create user
        let now = Utc::now();
        let user = User {
            id: Uuid::new_v4(),
            name: input.name,
            email: input.email,
            password_hash,
            phone: input.phone,
            role: UserRole::Student,
            created_at: now,
            updated_at: now,
        };

        let created_user = self.repo.create_user(&user)?;

        // Create student profile
        let profile = StudentProfile {
            user_id: created_user.id,
            grade: input.grade,
            mother_tongue: input.mother_tongue,
            state_of_residence: input.state_of_residence,
            board: input.board,
            school_id: input.school_id,
            parent_email: input.parent_email,
        };
        self.repo.create_student_profile(&profile)?;

        // Generate tokens
        let tokens = self.generate_token_pair(created_user.id, created_user.role)?;

        Ok((created_user.id, tokens))
    }

    /// Register a new teacher account.
    pub fn register_teacher(
        &self,
        input: RegisterTeacherInput,
    ) -> Result<(Uuid, TokenPair), AuthError> {
        if self.repo.find_by_email(&input.email)?.is_some() {
            return Err(AuthError::EmailAlreadyExists(input.email));
        }

        let password_hash = self.hash_password(&input.password)?;

        let now = Utc::now();
        let user = User {
            id: Uuid::new_v4(),
            name: input.name,
            email: input.email,
            password_hash,
            phone: input.phone,
            role: UserRole::Teacher,
            created_at: now,
            updated_at: now,
        };

        let created_user = self.repo.create_user(&user)?;

        let profile = TeacherProfile {
            user_id: created_user.id,
            languages: input.languages,
            qualifications: input.qualifications,
            bio: input.bio,
            verified: false,
            rating: None,
        };
        self.repo.create_teacher_profile(&profile)?;

        let tokens = self.generate_token_pair(created_user.id, created_user.role)?;

        Ok((created_user.id, tokens))
    }

    /// Authenticate a user with email and password.
    pub fn login(
        &self,
        email: &str,
        password: &str,
    ) -> Result<(Uuid, UserRole, TokenPair), AuthError> {
        let user = self
            .repo
            .find_by_email(email)?
            .ok_or(AuthError::InvalidCredentials)?;

        self.verify_password(password, &user.password_hash)?;

        let tokens = self.generate_token_pair(user.id, user.role)?;

        Ok((user.id, user.role, tokens))
    }

    /// Get a user's profile by ID.
    pub fn get_profile(&self, user_id: Uuid) -> Result<User, AuthError> {
        self.repo
            .find_by_id(user_id)?
            .ok_or_else(|| AuthError::UserNotFound(user_id.to_string()))
    }

    /// Update a user's profile.
    pub fn update_profile(
        &self,
        user_id: Uuid,
        input: UpdateProfileInput,
    ) -> Result<(), AuthError> {
        // Verify user exists
        self.repo
            .find_by_id(user_id)?
            .ok_or_else(|| AuthError::UserNotFound(user_id.to_string()))?;

        self.repo.update_user(user_id, &input)
    }

    // ── Private helpers ─────────────────────────────────────────

    fn hash_password(&self, password: &str) -> Result<String, AuthError> {
        use argon2::{
            password_hash::{rand_core::OsRng, SaltString},
            Argon2, PasswordHasher,
        };

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| AuthError::PasswordHashError(e.to_string()))
    }

    fn verify_password(
        &self,
        password: &str,
        hash: &str,
    ) -> Result<(), AuthError> {
        use argon2::{
            password_hash::PasswordHash, Argon2, PasswordVerifier,
        };

        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AuthError::PasswordHashError(e.to_string()))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AuthError::InvalidCredentials)
    }

    fn generate_token_pair(
        &self,
        user_id: Uuid,
        role: UserRole,
    ) -> Result<TokenPair, AuthError> {
        use jsonwebtoken::{encode, EncodingKey, Header};
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        struct Claims {
            sub: String,
            role: String,
            exp: usize,
            iat: usize,
        }

        let now = Utc::now().timestamp() as usize;

        // Access token — 1 hour
        let access_claims = Claims {
            sub: user_id.to_string(),
            role: format!("{:?}", role),
            exp: now + 3600,
            iat: now,
        };

        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AuthError::InvalidToken(e.to_string()))?;

        // Refresh token — 30 days
        let refresh_claims = Claims {
            sub: user_id.to_string(),
            role: format!("{:?}", role),
            exp: now + 30 * 24 * 3600,
            iat: now,
        };

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AuthError::InvalidToken(e.to_string()))?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repository::MockUserRepository;
    use shared::types::Grade;

    fn make_service() -> AuthServiceImpl<MockUserRepository> {
        let mut mock_repo = MockUserRepository::new();

        // Default: no existing user
        mock_repo
            .expect_find_by_email()
            .returning(|_| Ok(None));

        mock_repo.expect_create_user().returning(|user| {
            Ok(user.clone())
        });

        mock_repo
            .expect_create_student_profile()
            .returning(|_| Ok(()));

        mock_repo
            .expect_create_teacher_profile()
            .returning(|_| Ok(()));

        AuthServiceImpl::new(mock_repo, "test-secret".to_string())
    }

    #[test]
    fn test_register_student_success() {
        let service = make_service();

        let input = RegisterStudentInput {
            name: "Arjun".to_string(),
            email: "arjun@example.com".to_string(),
            password: "securepass123".to_string(),
            phone: "+919876543210".to_string(),
            grade: Grade::new(7).unwrap(),
            mother_tongue: "Tamil".to_string(),
            state_of_residence: "Karnataka".to_string(),
            board: None,
            school_id: None,
            parent_email: None,
        };

        let result = service.register_student(input);
        assert!(result.is_ok());

        let (user_id, tokens) = result.unwrap();
        assert!(!tokens.access_token.is_empty());
        assert!(!tokens.refresh_token.is_empty());
        assert_ne!(user_id, Uuid::nil());
    }

    #[test]
    fn test_register_student_duplicate_email() {
        let mut mock_repo = MockUserRepository::new();

        // Email already exists
        mock_repo.expect_find_by_email().returning(|_| {
            Ok(Some(User {
                id: Uuid::new_v4(),
                name: "Existing".to_string(),
                email: "arjun@example.com".to_string(),
                password_hash: "hash".to_string(),
                phone: "+919876543210".to_string(),
                role: UserRole::Student,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            }))
        });

        let service =
            AuthServiceImpl::new(mock_repo, "test-secret".to_string());

        let input = RegisterStudentInput {
            name: "Arjun".to_string(),
            email: "arjun@example.com".to_string(),
            password: "securepass123".to_string(),
            phone: "+919876543210".to_string(),
            grade: Grade::new(7).unwrap(),
            mother_tongue: "Tamil".to_string(),
            state_of_residence: "Karnataka".to_string(),
            board: None,
            school_id: None,
            parent_email: None,
        };

        let result = service.register_student(input);
        assert!(matches!(result, Err(AuthError::EmailAlreadyExists(_))));
    }
}
