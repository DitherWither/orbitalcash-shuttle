use secrecy::Secret;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub display_name: String,
    pub email: String, 
    /// "None" when password is hidden
    #[serde(skip_serializing, skip_deserializing)]
    pub password: Option<Secret<String>>, 
    #[serde(skip_serializing, skip_deserializing)]
    pub creation_time: chrono::NaiveDateTime,
}
