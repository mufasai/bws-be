use crate::models::aplikasi::{Aplikasi, CreateAplikasi};
use crate::models::response::ApiResponse;
use actix_web::{web, HttpResponse, Responder};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub async fn create_aplikasi(
    db: web::Data<Surreal<Client>>,
    aplikasi: web::Json<CreateAplikasi>,
) -> impl Responder {
    let app_name = aplikasi.app_name.clone();
    let status = aplikasi.status.clone();
    let api_key = aplikasi.api_key.clone();

    let sql = r#"
        CREATE aplikasi 
        SET 
            app_name = $app_name,
            status = $status,
            api_key = $api_key,
            created_at = time::now(), 
            updated_at = time::now()
    "#;

    let result = db
        .query(sql)
        .bind(("app_name", app_name))
        .bind(("status", status))
        .bind(("api_key", api_key))
        .await;

    match result {
        Ok(mut response) => match response.take::<Option<Aplikasi>>(0) {
            Ok(Some(aplikasi)) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "Aplikasi created successfully".to_string(),
                data: Some(aplikasi),
            }),
            Ok(None) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "Failed to create aplikasi".to_string(),
                data: None,
            }),
            Err(e) => {
                log::error!("Error processing create aplikasi response: {}", e);
                HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: e.to_string(),
                    data: None,
                })
            }
        },
        Err(e) => {
            log::error!("Database error in create aplikasi: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: e.to_string(),
                data: None,
            })
        }
    }
}

pub async fn get_aplikasi(db: web::Data<Surreal<Client>>) -> impl Responder {
    let result = db.query("SELECT * FROM aplikasi").await;

    match result {
        Ok(mut response) => match response.take::<Vec<Aplikasi>>(0) {
            Ok(aplikasi) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "Aplikasi retrieved successfully".to_string(),
                data: Some(aplikasi),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: format!("Failed to process aplikasi: {}", e),
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

pub async fn update_aplikasi(
    db: web::Data<Surreal<Client>>,
    aplikasi_id: web::Path<String>,
    aplikasi: web::Json<CreateAplikasi>,
) -> impl Responder {
    let thing = Thing::from(("aplikasi", aplikasi_id.as_str()));
    let app_name = aplikasi.app_name.clone();
    let status = aplikasi.status.clone();
    let api_key = aplikasi.api_key.clone();

    let sql = r#"
        UPDATE $thing 
        SET 
            app_name = $app_name, 
            status = $status,
            api_key = $api_key,
            updated_at = time::now()
    "#;

    let result = db
        .query(sql)
        .bind(("thing", thing))
        .bind(("app_name", app_name))
        .bind(("status", status))
        .bind(("api_key", api_key))
        .await;

    match result {
        Ok(mut response) => match response.take::<Option<Aplikasi>>(0) {
            Ok(Some(aplikasi)) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "Aplikasi updated successfully".to_string(),
                data: Some(aplikasi),
            }),
            Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "Aplikasi not found".to_string(),
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

pub async fn delete_aplikasi(
    db: web::Data<Surreal<Client>>,
    aplikasi_id: web::Path<String>,
) -> impl Responder {
    let thing = Thing::from(("aplikasi", aplikasi_id.as_str()));

    let result = db.query("DELETE $thing").bind(("thing", thing)).await;

    match result {
        Ok(mut response) => match response.take::<Vec<Aplikasi>>(0) {
            Ok(deleted_aplikasi) => {
                if deleted_aplikasi.is_empty() {
                    HttpResponse::Ok().json(ApiResponse::<()> {
                        status: "success".to_string(),
                        message: "Aplikasi deleted successfully".to_string(),
                        data: None,
                    })
                } else {
                    HttpResponse::Ok().json(ApiResponse::<()> {
                        status: "success".to_string(),
                        message: "Aplikasi deleted successfully".to_string(),
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
