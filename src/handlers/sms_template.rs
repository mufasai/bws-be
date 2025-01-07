use crate::models::response::ApiResponse;
use crate::models::sms_template::{CreateTemplate, TemplateSMS};
use actix_web::{web, HttpResponse, Responder};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub async fn create_template_sms(
    db: web::Data<Surreal<Client>>,
    sms: web::Json<CreateTemplate>,
) -> impl Responder {
    let judul = sms.judul.clone();
    let content_type = sms.content_type.clone();
    let content = sms.content.clone();
    let created_by = sms.created_by.clone();

    let sql = r#"
        CREATE sms_template 
        SET 
            judul = $judul,
            content_type = $content_type,
            content = $content,
            created_by = $created_by,
            created_at = time::now(), 
            updated_at = time::now()
    "#;

    let result = db
        .query(sql)
        .bind(("judul", judul))
        .bind(("content_type", content_type))
        .bind(("content", content))
        .bind(("created_by", created_by))
        .await;

    match result {
        Ok(mut response) => match response.take::<Option<TemplateSMS>>(0) {
            Ok(Some(sms)) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "SMS Template created successfully".to_string(),
                data: Some(sms),
            }),
            Ok(None) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "Failed to create sms template".to_string(),
                data: None,
            }),
            Err(e) => {
                log::error!("Error processing create user response: {}", e);
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

pub async fn get_template(db: web::Data<Surreal<Client>>) -> impl Responder {
    log::debug!("Attempting to get all SMS Template");
    let result = db.query("SELECT * FROM sms_template").await;

    match result {
        Ok(mut response) => {
            log::debug!("Query successful, processing response");
            match response.take::<Vec<TemplateSMS>>(0) {
                Ok(sms) => {
                    log::debug!("Found {} sms template", sms.len());
                    HttpResponse::Ok().json(ApiResponse {
                        status: "success".to_string(),
                        message: "SMS Template retrieved successfully".to_string(),
                        data: Some(sms),
                    })
                }
                Err(e) => {
                    log::error!("Error taking sms Template from response: {}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        status: "error".to_string(),
                        message: format!("Failed to process SMS Template: {}", e),
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

pub async fn update_sms_template(
    db: web::Data<Surreal<Client>>,
    sms_id: web::Path<String>,
    sms: web::Json<CreateTemplate>,
) -> impl Responder {
    let thing = Thing::from(("sms_template", sms_id.as_str()));
    let judul = sms.judul.clone();
    let content_type = sms.content_type.clone();
    let content = sms.content.clone();
    let created_by = sms.created_by.clone();

    let sql = r#"
        UPDATE $thing 
        SET 
            judul = $judul, 
            content_type = $content_type,
            content = $content,
            created_by = $created_by,
            updated_at = time::now()
    "#;

    let result = db
        .query(sql)
        .bind(("thing", thing))
        .bind(("judul", judul))
        .bind(("content_type", content_type))
        .bind(("content", content))
        .bind(("created_by", created_by))
        .await;

    match result {
        Ok(mut response) => match response.take::<Option<TemplateSMS>>(0) {
            Ok(Some(sms)) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "SMS Template updated successfully".to_string(),
                data: Some(sms),
            }),
            Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "SMS Template not found".to_string(),
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

pub async fn delete_sms_template(
    db: web::Data<Surreal<Client>>,
    sms_id: web::Path<String>,
) -> impl Responder {
    let thing = Thing::from(("sms_template", sms_id.as_str()));

    let result = db.query("DELETE $thing").bind(("thing", thing)).await;

    match result {
        Ok(mut response) => match response.take::<Vec<TemplateSMS>>(0) {
            Ok(deleted_sms_template) => {
                if deleted_sms_template.is_empty() {
                    HttpResponse::Ok().json(ApiResponse::<()> {
                        status: "success".to_string(),
                        message: "SMS Template deleted successfully".to_string(),
                        data: None,
                    })
                } else {
                    HttpResponse::Ok().json(ApiResponse::<()> {
                        status: "success".to_string(),
                        message: "SMS Template deleted successfully".to_string(),
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
