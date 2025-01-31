use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShowFile {
    pub name: String,
    pub file_type: String,
    pub status: String,
    pub uploaded_at: String, // Use ISO 8601 format
    pub file_data: Vec<u8>, // Binary file data
}
