// src/models/sms_cost.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsCost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Thing>,
    pub provider: String,
    pub cost: String,
    pub currency: String,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSmsCost {
    pub provider: String,
    pub cost: String,
    pub currency: String,
}
