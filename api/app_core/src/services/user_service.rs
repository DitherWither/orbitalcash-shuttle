use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use secrecy::Secret;
use serde::Deserialize;
use sqlx::PgPool;
use thiserror::Error;

use crate::models::User;

#[derive(Debug, Error)]
pub enum UserGetError {
    #[error("Database Error: `{0}`")]
    DatabaseError(sqlx::Error),

    #[error("User with id `{0}` does not exist")]
    DoesNotExist(i32),
}

pub struct UserService {
    db: PgPool,
}

impl UserService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    /// Returns all users, without tags and passwords
    pub async fn get_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users =
            sqlx::query!("SELECT user_id, display_name, email, password, creation_time FROM users")
                .fetch_all(&self.db)
                .await?;

        let users: Vec<User> = users
            .iter()
            .map(|e| User {
                user_id: e.user_id,
                display_name: e.display_name.clone(),
                email: e.email.clone(),
                password: Some(Secret::new(e.password.clone())),
                creation_time: e.creation_time,
            })
            .collect();

        Ok(users)
    }

    pub async fn get_by_id(&self, user_id: i32) -> Result<User, UserGetError> {
        let user = sqlx::query!(
            "SELECT user_id, display_name, email, password, creation_time FROM users WHERE user_id = $1",
            user_id
        ).fetch_optional(&self.db).await.map_err(UserGetError::DatabaseError)?;

        match user {
            Some(user) => Ok(User {
                user_id: user.user_id,
                display_name: user.display_name,
                email: user.email,
                password: Some(Secret::new(user.password)),
                creation_time: user.creation_time,
            }),
            None => Err(UserGetError::DoesNotExist(user_id)),
        }
    }

    pub async fn register(&self, body: &RegisterBody) -> Result<i32, RegisterError> {
        // Check if email is already in use
        let user = sqlx::query!("SELECT user_id FROM users WHERE email = $1", body.email)
            .fetch_optional(&self.db)
            .await
            .map_err(RegisterError::DatabaseError)?;

        if user.is_some() {
            return Err(RegisterError::EmailAlreadyInUse);
        }

        // Hash and salt the password
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        // Hash the password into a phc String
        let password_hash = argon2
            .hash_password(body.password.as_bytes(), &salt)
            .map_err(RegisterError::PasswordHashError)?
            .to_string();

        // Create the user in the database
        sqlx::query!(
            "INSERT INTO users (display_name, email, password) VALUES ($1, $2, $3)",
            body.display_name,
            body.email,
            password_hash
        )
        .execute(&self.db)
        .await
        .map_err(RegisterError::DatabaseError)?;

        // Get the user id
        let user = sqlx::query!("SELECT user_id FROM users WHERE email = $1", body.email)
            .fetch_one(&self.db)
            .await
            .map_err(RegisterError::DatabaseError)?;

        Ok(user.user_id)
    }

    pub async fn login(&self, body: &LoginBody) -> Result<i32, LoginError> {
        let db_user = sqlx::query!(
            "SELECT user_id, email, password FROM users WHERE email = $1",
            body.email
        )
        .fetch_optional(&self.db)
        .await
        .map_err(LoginError::DatabaseError)?;

        if db_user.is_none() {
            return Err(LoginError::InvalidUser);
        }

        let db_user = db_user.unwrap();

        // verify the password
        let argon2 = Argon2::default();

        let parsed_hash =
            PasswordHash::new(&db_user.password).map_err(LoginError::PasswordHashError)?;

        argon2
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_err(|_| LoginError::InvalidPassword)?;

        // Return the user id, as the login was successful
        Ok(db_user.user_id)
    }
}

#[derive(Deserialize)]
pub struct RegisterBody {
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Error)]
pub enum RegisterError {
    #[error("Email already in use")]
    EmailAlreadyInUse,

    #[error("Database Error: `{0}`")]
    DatabaseError(sqlx::Error),

    #[error("Password Hash Error: `{0}`")]
    PasswordHashError(argon2::password_hash::Error),
}

#[derive(Deserialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("User does not exist")]
    InvalidUser,

    #[error("Wrong Password")]
    InvalidPassword,

    #[error("Password Hash Error: `{0}`")]
    PasswordHashError(argon2::password_hash::Error),

    #[error("Database Error: `{0}`")]
    DatabaseError(sqlx::Error),
}
