use actix_web::{web, HttpResponse, Responder};
use crate::models::inbox::InboxMessage;
use crate::models::response::ApiResponse;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;

pub async fn get_inbox(
    db: web::Data<Surreal<Client>>,
    path: web::Path<String>, // Untuk menerima user_id dari URL
) -> impl Responder {
    let user_id = path.into_inner();
    let query = "SELECT * FROM inbox_message WHERE user_id = $user_id ORDER BY created_at DESC";
    
    match db.query(query).bind(("user_id", user_id)).await {
        Ok(mut result) => {
            let inbox_messages = result.take::<Vec<InboxMessage>>(0).unwrap_or_else(|_| vec![]);
            HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "Inbox messages fetched successfully".to_string(),
                data: Some(inbox_messages),
            })
        }
        Err(e) => {
            log::error!("Failed to fetch inbox messages: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: format!("Failed to fetch inbox messages: {}", e),
                data: None,
            })
        }
    }
}