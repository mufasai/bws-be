use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use surrealdb::{Surreal, engine::remote::ws::Client};
use tracing::{error, info};
use chrono::{self, DateTime, Utc};
use surrealdb::sql::Thing;




// Struct for SMS input, only needs phone_number and message
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SmsInput {
    pub number: String,
    pub message: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct SmsRecord {
    #[serde(default)]
    pub id: Option<String>,
    pub number: String,
    pub message: String,
    #[serde(default = "default_status")]
    pub status: String,
    #[serde(default = "default_created_at")]
    pub created_at: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SmsInputWithStatus {
    pub number: String,
    pub message: String,
    #[serde(default = "default_status")]  // Default status value
    pub status: String,
    
    #[serde(default = "default_created_at")]  // Default timestamp
    pub created_at: String,
}

// Struct for the response after successful creation
#[derive(Serialize, Debug)]
pub struct SmsResponse {
    pub status: String,
    pub message: String,
}

// Default function for status
fn default_status() -> String {
    "pending".to_string()  // Default status is "pending"
}

fn convert_to_datetime(date_str: &str) -> DateTime<Utc> {
    // Gunakan chrono untuk mengonversi string ke DateTime
    DateTime::parse_from_rfc3339(date_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now()) // Menggunakan waktu sekarang jika konversi gagal
}

// Default function for created_at
fn default_created_at() -> String {
    // Generate current UTC time as RFC3339 string
    Utc::now().to_rfc3339()  // Use the current UTC time formatted correctly
}

// Function to handle JSON file upload
pub async fn upload_json_file(
    payload: web::Json<Vec<SmsRecord>>, // Payload array SmsRecord
    db: web::Data<Surreal<Client>>,    // Koneksi ke SurrealDB
) -> Result<HttpResponse, Error> {
    info!("Menerima {} data untuk diunggah", payload.len());

    let mut successful_records = Vec::new();
    let mut errors = Vec::new();

    for record in payload.into_inner() {
        // Validasi data
        if record.number.is_empty() || record.message.is_empty() {
            let error_msg = format!(
                "Invalid data: number='{}', message='{}'",
                record.number, record.message
            );
            error!("{}", error_msg);
            errors.push(error_msg);
            continue;
        }

        // Pastikan `created_at` memiliki format yang benar jika belum diisi
        let created_at = if record.created_at.is_empty() {
            default_created_at()  // Gunakan default jika kosong
        } else {
            // Konversi `created_at` ke DateTime dan ubah menjadi string yang sesuai dengan SurrealDB
            convert_to_datetime(&record.created_at).to_rfc3339()
        };

        // Query untuk memasukkan data ke SurrealDB
        let query = "CREATE sms SET number = $number, message = $message, status = $status, created_at = $created_at RETURN *";
        
        match db
            .query(query)
            .bind(("number", record.number.clone()))
            .bind(("message", record.message.clone()))
            .bind(("status", record.status.clone()))
            .bind(("created_at", created_at))
            .await
        {
            Ok(mut response) => {
                match response.take::<Vec<SmsRecord>>(0) {
                    Ok(created) => {
                        info!("Created records: {:?}", created);
                        if let Some(record) = created.first() {
                            successful_records.push(record.clone());
                            info!("Successfully saved record: {}", record.number);
                        }
                    }
                    Err(e) => {
                        error!("Error retrieving response: {}", e);
                        errors.push(format!("Error retrieving response: {}", e));
                    }
                }
            }
            Err(e) => {
                let error_msg = format!("Kesalahan database: {}", e);
                error!("{}", error_msg);
                errors.push(error_msg);
            }
        }
    }

    // Tentukan status berdasarkan apakah ada error atau tidak
    let status = if errors.is_empty() { "success" } else { "partial" };

    let response = json!( {
        "status": status,
        "success_count": successful_records.len(),
        "errors_count": errors.len(),
        "errors": errors,
        "records": successful_records,
    });

    Ok(HttpResponse::Ok().json(response))
}


// Function to handle SMS input via API
pub async fn input_sms_(
    db: web::Data<Surreal<Client>>,  // SurrealDB instance
    sms_input: web::Json<SmsInput>,  // Using SmsInput
) -> HttpResponse {
    let sms_data = sms_input.into_inner();

    // Prepare SMS data with default status and timestamp
    let sms_data_with_status = SmsInputWithStatus {
        number: sms_data.number,
        message: sms_data.message,
        status: default_status(),
        created_at: format!("{}", chrono::Utc::now()),  // Timestamp
    };

    // Ensure the query and bindings are correct
    let query = r#"
        CREATE sms SET
            number = $number,
            message = $message,
            status = $status,
            created_at = time::now()
        RETURN *
    "#;

    // Bind parameters with cloned values
    let result = db
        .as_ref()
        .query(query)
        .bind(("number", sms_data_with_status.number.clone()))
        .bind(("message", sms_data_with_status.message.clone()))
        .bind(("status", sms_data_with_status.status.clone()))
        .bind(("created_at", sms_data_with_status.created_at.clone()))
        .await;

    match result {
        Ok(mut response) => {
            // Log the response to debug the data being returned from the query
            info!("Query result: {:?}", response);

            // Check the response data explicitly
            if let Some(records) = response.take::<Vec<SmsInputWithStatus>>(0).ok() {
                info!("Records created: {:?}", records);
            } else {
                info!("No records returned from query.");
            }

            // Build the response body
            let response_body = SmsResponse {
                status: "success".to_string(),
                message: format!("{} successfully sent", sms_data_with_status.message),
            };
            HttpResponse::Ok().json(response_body)
        }
        Err(e) => {
            error!("Failed to save SMS: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Failed to save SMS: {:?}", e))
        }
    }
}


