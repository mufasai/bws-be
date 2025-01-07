use crate::models::country_code::{CountryCode, CreateCountryCode};
use crate::models::response::ApiResponse;
use actix_web::{web, HttpResponse, Responder};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub async fn create_country_code(
    db: web::Data<Surreal<Client>>,
    country_code: web::Json<CreateCountryCode>,
) -> impl Responder {
    let code_number = country_code.code_number.clone();
    let country = country_code.country.clone();
    let status = country_code.status.clone();

    let sql = r#"
        CREATE country_codes 
        SET 
            code_number = $code_number,
            country = $country,
            status = $status,
            created_at = time::now(), 
            updated_at = time::now()
    "#;

    let result = db
        .query(sql)
        .bind(("code_number", code_number))
        .bind(("country", country))
        .bind(("status", status))
        .await;

    match result {
        Ok(mut response) => match response.take::<Option<CountryCode>>(0) {
            Ok(Some(country_code)) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "Country code created successfully".to_string(),
                data: Some(country_code),
            }),
            Ok(None) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "Failed to create country code".to_string(),
                data: None,
            }),
            Err(e) => {
                log::error!("Error processing create country code response: {}", e);
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

pub async fn get_country_code(db: web::Data<Surreal<Client>>) -> impl Responder {
    let result = db.query("SELECT * FROM country_codes").await;

    match result {
        Ok(mut response) => match response.take::<Vec<CountryCode>>(0) {
            Ok(country_code) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "Country code retrieved successfully".to_string(),
                data: Some(country_code),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: format!("Failed to process country code: {}", e),
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

pub async fn update_country_code(
    db: web::Data<Surreal<Client>>,
    country_id: web::Path<String>,
    country_code: web::Json<CreateCountryCode>,
) -> impl Responder {
    let thing = Thing::from(("country_codes", country_id.as_str()));
    let code_number = country_code.code_number.clone();
    let country = country_code.country.clone();
    let status = country_code.status.clone();

    let sql = r#"
        UPDATE $thing 
        SET 
            code_number = $code_number, 
            country = $country,
            status = $status,
            updated_at = time::now()
    "#;

    let result = db
        .query(sql)
        .bind(("thing", thing))
        .bind(("code_number", code_number))
        .bind(("country", country))
        .bind(("status", status))
        .await;

    match result {
        Ok(mut response) => match response.take::<Option<CountryCode>>(0) {
            Ok(Some(country_code)) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "Country code updated successfully".to_string(),
                data: Some(country_code),
            }),
            Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "Country code not found".to_string(),
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

pub async fn delete_country_code(
    db: web::Data<Surreal<Client>>,
    country_id: web::Path<String>,
) -> impl Responder {
    let thing = Thing::from(("country_codes", country_id.as_str()));

    let result = db.query("DELETE $thing").bind(("thing", thing)).await;

    match result {
        Ok(mut response) => match response.take::<Vec<CountryCode>>(0) {
            Ok(deleted_country_code) => {
                if deleted_country_code.is_empty() {
                    HttpResponse::Ok().json(ApiResponse::<()> {
                        status: "success".to_string(),
                        message: "Country code deleted successfully".to_string(),
                        data: None,
                    })
                } else {
                    HttpResponse::Ok().json(ApiResponse::<()> {
                        status: "success".to_string(),
                        message: "Country code deleted successfully".to_string(),
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
