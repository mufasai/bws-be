use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateSMS {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub judul: String,
    pub content_type: String,
    pub content: String,
    pub created_by: String,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTemplate {
    pub judul: String,
    pub content_type: String,
    pub content: String,
    pub created_by: String,
}
