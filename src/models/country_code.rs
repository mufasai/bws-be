use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct CountryCode {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub code_number: String,
    pub country: String,
    pub status: String,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCountryCode {
    pub code_number: String,
    pub country: String,
    pub status: String,
}
