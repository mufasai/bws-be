use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Aplikasi {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub app_name: String,
    pub status: String,
    pub api_key: String,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAplikasi {
    pub app_name: String,
    pub status: String,
    pub api_key: String,
}
