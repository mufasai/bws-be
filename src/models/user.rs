use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub role: Option<Thing>,
    pub fullname: String,
    pub address: String,
    pub country: String,
    pub city: String,
    pub state: String,
    pub country_code: String,
    pub verification_status: String,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinishingUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub role_id: Option<Thing>,
    pub role_name: String,
    pub role_description: String,
    pub fullname: String,
    pub address: String,
    pub country: String,
    pub city: String,
    pub state: String,
    pub country_code: String,
    pub verification_status: String,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub role: UserRole,
    pub fullname: String,
    pub address: String,
    pub country: String,
    pub city: String,
    pub state: String,
    pub country_code: String,
    pub verification_status: String,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRole {
    pub id: Option<Thing>,
    pub role_name: String,
    pub role_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub role: String,
    pub fullname: String,
    pub address: String,
    pub country: String,
    pub city: String,
    pub state: String,
    pub country_code: String,
    pub verification_status: String,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
