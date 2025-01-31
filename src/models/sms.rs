use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct OTP {
    pub id: Option<Thing>,
    pub user_id: String,
    pub code: String,
    pub expires_at: String,
    pub is_used: bool,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SMSTemplate {
    pub id: Option<Thing>,
    pub judul: String,
    pub content: String,
    pub content_type: String,
    pub created_at: Option<String>,
    pub created_by: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SMSRequest {
    pub user_id: String,      // Menyesuaikan dengan field "user_id"
    pub phone_number: String, // Menyesuaikan dengan field "phone_number"
    pub template_id: String,  // Menyesuaikan dengan field "template_id"
}
