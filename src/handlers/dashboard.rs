use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use crate::models::response::ApiResponse;

#[derive(Debug, Serialize)]
pub struct DashboardSummary {
    total_users: i64,
    active_users: i64,
}

pub async fn get_summary(db: web::Data<Surreal<Client>>) -> impl Responder {
    let result = db
        .query("SELECT count() as total FROM users GROUP ALL")
        .await;

    match result {
        Ok(mut response) => {
            #[derive(Debug, Deserialize)]
            struct Count {
                total: i64,
            }

            match response.take::<Vec<Count>>(0) {
                Ok(counts) => {
                    let total = counts.first().map(|c| c.total).unwrap_or(0);
                    let summary = DashboardSummary {
                        total_users: total,
                        active_users: total, // For now, same as total
                    };
                    HttpResponse::Ok().json(ApiResponse {
                        status: "success".to_string(),
                        message: "Dashboard summary retrieved successfully".to_string(),
                        data: Some(summary),
                    })
                }
                Err(e) => {
                    log::error!("Error taking count from response: {}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        status: "error".to_string(),
                        message: format!("Failed to process count: {}", e),
                        data: None,
                    })
                }
            }
        }
        Err(e) => {
            log::error!("Database query error: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: format!("Database error: {}", e),
                data: None,
            })
        }
    }
} 