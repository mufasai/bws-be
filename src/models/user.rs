use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use validator::Validate;

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

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub verification_code: String,
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct ForgotPasswordResponse {
    pub message: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLogreg {
    pub id: Thing,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub verification_code: String,
    #[serde(default)]
    pub is_verified: bool,
    pub dob: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String, // Hanya menerima token sebagai query parameter
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationQuery {
    pub token: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct VerifyCodeRequest {
    pub email: String,
    pub code: String,
}

#[derive(Serialize)]
pub struct VerificationResponse {
    pub email: String,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegUser {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub dob: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginResponse {
    pub token: String,
    pub message: String,
    pub id : Thing
    // pub data : Option<User>,
    // pub(crate) id: Thing, // Make this field public
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMS {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub phone_number: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct InputSMSRequest {
    #[validate(length(min = 10, max = 15, message = "Nomor telepon harus antara 10-15 digit"))]
    pub phone_number: String,
    #[validate(length(min = 1, max = 160, message = "Pesan harus antara 1-160 karakter"))]
    pub message: String,
}
