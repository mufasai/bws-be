use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "id")]
    pub role_id: Option<Thing>,
    pub role_name: String,
    pub role_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroup {
    pub role_name: String,
    pub role_description: String,
}