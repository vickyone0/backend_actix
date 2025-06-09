use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}



#[derive(Debug, Deserialize, Validate)] // Add validation
pub struct CreateUser {
    #[validate(length(min = 3, max = 24))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
}