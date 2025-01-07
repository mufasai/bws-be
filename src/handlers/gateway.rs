use crate::models::gateway::{CreateGateway, Gateway};
use crate::models::response::ApiResponse;
use actix_web::{web, HttpResponse, Responder};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub async fn create_gateway(
    db: web::Data<Surreal<Client>>,
    gateway: web::Json<CreateGateway>,
) -> impl Responder {
    let app_name = gateway.app_name.clone();
    let status = gateway.status.clone();
    let priority = gateway.priority.clone();
    let failover = gateway.failover.clone();

    let sql = r#"
        CREATE gateway 
        SET 
            app_name = $app_name,
            status = $status,
            priority = $priority,
            failover = $failover,
            created_at = time::now(), 
            updated_at = time::now()
    "#;

    let result = db
        .query(sql)
        .bind(("app_name", app_name))
        .bind(("status", status))
        .bind(("priority", priority))
        .bind(("failover", failover))
        .await;

    match result {
        Ok(mut response) => match response.take::<Option<Gateway>>(0) {
            Ok(Some(gateway)) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "Gateway created successfully".to_string(),
                data: Some(gateway),
            }),
            Ok(None) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "Failed to create gateway".to_string(),
                data: None,
            }),
            Err(e) => {
                log::error!("Error processing create gateway response: {}", e);
                HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: e.to_string(),
                    data: None,
                })
            }
        },
        Err(e) => {
            log::error!("Database error in create gateway: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: e.to_string(),
                data: None,
            })
        }
    }
}

pub async fn get_gateway(db: web::Data<Surreal<Client>>) -> impl Responder {
    let result = db.query("SELECT * FROM gateway").await;

    match result {
        Ok(mut response) => match response.take::<Vec<Gateway>>(0) {
            Ok(gateway) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "Gateway retrieved successfully".to_string(),
                data: Some(gateway),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: format!("Failed to process gateway: {}", e),
                data: None,
            }),
        },
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: format!("Database error: {}", e),
            data: None,
        }),
    }
}

pub async fn update_gateway(
    db: web::Data<Surreal<Client>>,
    gateway_id: web::Path<String>,
    gateway: web::Json<CreateGateway>,
) -> impl Responder {
    let thing = Thing::from(("gateway", gateway_id.as_str()));
    let app_name = gateway.app_name.clone();
    let status = gateway.status.clone();
    let priority = gateway.priority.clone();
    let failover = gateway.failover.clone();

    let sql = r#"
        UPDATE $thing 
        SET 
            app_name = $app_name, 
            status = $status,
            priority = $priority,
            failover = $failover,
            updated_at = time::now()
    "#;

    let result = db
        .query(sql)
        .bind(("thing", thing))
        .bind(("app_name", app_name))
        .bind(("status", status))
        .bind(("priority", priority))
        .bind(("failover", failover))
        .await;

    match result {
        Ok(mut response) => match response.take::<Option<Gateway>>(0) {
            Ok(Some(gateway)) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "Gateway updated successfully".to_string(),
                data: Some(gateway),
            }),
            Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "Gateway not found".to_string(),
                data: None,
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: e.to_string(),
                data: None,
            }),
        },
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: e.to_string(),
            data: None,
        }),
    }
}

pub async fn delete_gateway(
    db: web::Data<Surreal<Client>>,
    gateway_id: web::Path<String>,
) -> impl Responder {
    let thing = Thing::from(("gateway", gateway_id.as_str()));

    let result = db.query("DELETE $thing").bind(("thing", thing)).await;

    match result {
        Ok(mut response) => match response.take::<Vec<Gateway>>(0) {
            Ok(deleted_gateway) => {
                if deleted_gateway.is_empty() {
                    HttpResponse::Ok().json(ApiResponse::<()> {
                        status: "success".to_string(),
                        message: "Gateway deleted successfully".to_string(),
                        data: None,
                    })
                } else {
                    HttpResponse::Ok().json(ApiResponse::<()> {
                        status: "success".to_string(),
                        message: "Gateway deleted successfully".to_string(),
                        data: None,
                    })
                }
            }
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: e.to_string(),
                data: None,
            }),
        },
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: e.to_string(),
            data: None,
        }),
    }
}
