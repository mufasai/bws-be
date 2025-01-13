use actix_web::{web, HttpResponse, Responder};
use surrealdb::{engine::remote::ws::Client, Surreal, sql::Thing};
use crate::models::user::{InputSMSRequest, SMS};
use crate::models::response::ApiResponse;
use chrono::{DateTime, Utc};

// Struct internal untuk menangani response database
#[derive(Debug, serde::Deserialize)]
struct DBSMSResponse {
    #[serde(rename = "id")]
    id: Thing,
    phone_number: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created_at: Option<DateTime<Utc>>,
}

pub async fn input_sms_direct(
    sms: web::Json<InputSMSRequest>,
    db: web::Data<Surreal<Client>>,
) -> impl Responder {
    let new_sms = SMS {
        id: None,  // Biarkan SurrealDB yang generate ID
        phone_number: sms.phone_number.clone(),
        message: sms.message.clone(),
        status: None,  // Biarkan SurrealDB yang mengisi status dengan default 'PENDING'
        created_at: None,  // Biarkan SurrealDB yang mengisi created_at dengan waktu saat ini
    };

    match save_sms(new_sms, &db).await {
        Ok(Some(result)) => {
            let response = ApiResponse {
                status: "success".to_string(),
                message: "SMS berhasil disimpan".to_string(),
                data: Some(result),
            };
            HttpResponse::Ok().json(response)
        }
        Ok(None) => {
            let response: ApiResponse<String> = ApiResponse {
                status: "error".to_string(),
                message: "Gagal menyimpan SMS".to_string(),
                data: None,
            };
            HttpResponse::InternalServerError().json(response)
        }
        Err(err) => {
            let response: ApiResponse<String> = ApiResponse {
                status: "error".to_string(),
                message: format!("Gagal menyimpan SMS: {}", err),
                data: None,
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

async fn save_sms(sms: SMS, db: &Surreal<Client>) -> Result<Option<SMS>, Box<dyn std::error::Error>> {
    let db_response: Option<DBSMSResponse> = db
        .create("sms")
        .content(serde_json::json!( {
            "phone_number": sms.phone_number,
            "message": sms.message,
        }))
        .await?;

    Ok(db_response.map(|resp| SMS {
        id: Some(resp.id.to_string()),
        phone_number: resp.phone_number,
        message: resp.message,
        status: resp.status,
        created_at: resp.created_at,
    }))
}
