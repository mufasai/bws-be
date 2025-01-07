use crate::models::provider_prefixes::{CreateProviderPrefixes, ProviderPrefixes};
use crate::models::response::ApiResponse;
use actix_web::{web, HttpResponse, Responder};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub async fn create_provider_prefixes(
    db: web::Data<Surreal<Client>>,
    provider_prefixes: web::Json<CreateProviderPrefixes>,
) -> impl Responder {
    let prefix = provider_prefixes.prefix.clone();
    let provider = provider_prefixes.provider.clone();
    let status = provider_prefixes.status.clone();

    let sql = r#"
        CREATE provider_prefixes 
        SET 
            prefix = $prefix,
            provider = $provider,
            status = $status,
            created_at = time::now(), 
            updated_at = time::now()
    "#;

    let result = db
        .query(sql)
        .bind(("prefix", prefix))
        .bind(("provider", provider))
        .bind(("status", status))
        .await;

    match result {
        Ok(mut response) => match response.take::<Option<ProviderPrefixes>>(0) {
            Ok(Some(provider_prefixes)) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "Provider prefixes created successfully".to_string(),
                data: Some(provider_prefixes),
            }),
            Ok(None) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "Failed to create provider prefixes".to_string(),
                data: None,
            }),
            Err(e) => {
                log::error!("Error processing create provider prefixess response: {}", e);
                HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: e.to_string(),
                    data: None,
                })
            }
        },
        Err(e) => {
            log::error!("Database error in create user: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: e.to_string(),
                data: None,
            })
        }
    }
}

pub async fn get_provider_prefixes(db: web::Data<Surreal<Client>>) -> impl Responder {
    log::debug!("Attempting to get all provider prefixes");
    let result = db.query("SELECT * FROM provider_prefixes").await;

    match result {
        Ok(mut response) => {
            log::debug!("Query successful, processing response");
            match response.take::<Vec<ProviderPrefixes>>(0) {
                Ok(provider_prefixes) => {
                    log::debug!("Found {} provider prefixes", provider_prefixes.len());
                    HttpResponse::Ok().json(ApiResponse {
                        status: "success".to_string(),
                        message: "Provider prefixes retrieved successfully".to_string(),
                        data: Some(provider_prefixes),
                    })
                }
                Err(e) => {
                    log::error!("Error taking provider prefixes from response: {}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        status: "error".to_string(),
                        message: format!("Failed to process Provider Prefixes response: {}", e),
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
pub async fn update_provider_prefixes(
    db: web::Data<Surreal<Client>>,
    prefix_id: web::Path<String>,
    provider_prefixes: web::Json<CreateProviderPrefixes>,
) -> impl Responder {
    let thing = Thing::from(("provider_prefixes", prefix_id.as_str()));
    let prefix = provider_prefixes.prefix.clone();
    let provider = provider_prefixes.provider.clone();
    let status = provider_prefixes.status.clone();

    let sql = r#"
        UPDATE $thing 
        SET 
            prefix = $prefix, 
            provider = $provider,
            status = $status,
            updated_at = time::now()
    "#;

    let result = db
        .query(sql)
        .bind(("thing", thing))
        .bind(("prefix", prefix))
        .bind(("provider", provider))
        .bind(("status", status))
        .await;

    match result {
        Ok(mut response) => match response.take::<Option<ProviderPrefixes>>(0) {
            Ok(Some(provider_prefixes)) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "Provider prefixes updated successfully".to_string(),
                data: Some(provider_prefixes),
            }),
            Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "Provider Prefixes not found".to_string(),
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

pub async fn delete_provider_prefixes(
    db: web::Data<Surreal<Client>>,
    prefix_id: web::Path<String>,
) -> impl Responder {
    let thing = Thing::from(("provider_prefixes", prefix_id.as_str()));

    let result = db.query("DELETE $thing").bind(("thing", thing)).await;

    match result {
        Ok(mut response) => match response.take::<Vec<ProviderPrefixes>>(0) {
            Ok(deleted_provider_prefixes) => {
                if deleted_provider_prefixes.is_empty() {
                    HttpResponse::Ok().json(ApiResponse::<()> {
                        status: "success".to_string(),
                        message: "Provider prefixes deleted successfully".to_string(),
                        data: None,
                    })
                } else {
                    HttpResponse::Ok().json(ApiResponse::<()> {
                        status: "success".to_string(),
                        message: "Provider prefixes deleted successfully".to_string(),
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
