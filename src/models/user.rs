use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub username: String,
    pub email: String,
    pub password: String,
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
    pub token: String,  // Hanya menerima token sebagai query parameter
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationQuery {
    pub token: String,
    pub email: String,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub message: String, // Make this field public
}
