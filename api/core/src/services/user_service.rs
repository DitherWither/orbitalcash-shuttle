use secrecy::Secret;
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
        let users = sqlx::query!("SELECT user_id, display_name, email, password, creation_time FROM users")
            .fetch_all(&self.db)
            .await?;

        let users: Vec<User> = users
            .iter()
            .map(|e| User {
                user_id: e.user_id,
                display_name: e.display_name.clone(),
                email: e.email.clone(),
                password: Some(Secret::new(e.password.clone())),
                creation_time: e.creation_time
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
            Some(user) => {
                Ok(
                    User {
                        user_id: user.user_id,
                        display_name: user.display_name,
                        email: user.email,
                        password: Some(Secret::new(user.password)),
                        creation_time: user.creation_time,
                    }
                )
            }
            None => Err(UserGetError::DoesNotExist(user_id)),
        }
    }
}
