use surrealdb::sql::Thing;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InboxMessage {
    pub id: Option<Thing>,              // ID yang dihasilkan oleh SurrealDB
    pub user_id: String,                // ID pengguna yang menerima pesan
    pub judul: String,                  // Judul pesan
    pub content: String,                // Isi pesan
    pub content_type: String,           // Tipe konten pesan, seperti 'text/plain'
    pub created_at: Option<String>,     // Tanggal dan waktu pembuatan pesan
    pub created_by: String,             // Pengguna yang membuat pesan, misalnya 'admin'
}
