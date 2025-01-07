use crate::models::response::ApiResponse;
use crate::models::sms_costs::{CreateSmsCost, SmsCost};
use actix_web::{web, HttpResponse, Responder};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub async fn create_sms_cost(
    db: web::Data<Surreal<Client>>,
    sms_cost: web::Json<CreateSmsCost>,
) -> impl Responder {
    let provider = sms_cost.provider.clone();
    let cost = sms_cost.cost.clone();
    let currency = sms_cost.currency.clone();

    let sql = r#"
        CREATE sms_cost 
        SET 
            provider = $provider,
            cost = $cost,
            currency = $currency,
            created_at = time::now(), 
            updated_at = time::now()
    "#;

    let result = db
        .query(sql)
        .bind(("provider", provider))
        .bind(("cost", cost))
        .bind(("currency", currency))
        .await;

    match result {
        Ok(mut response) => match response.take::<Option<SmsCost>>(0) {
            Ok(Some(sms_cost)) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "SMS cost created successfully".to_string(),
                data: Some(sms_cost),
            }),
            Ok(None) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "Failed to create sms cost".to_string(),
                data: None,
            }),
            Err(e) => {
                log::error!("Error processing create sms cost response: {}", e);
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

pub async fn get_sms_cost(db: web::Data<Surreal<Client>>) -> impl Responder {
    let result = db.query("SELECT * FROM sms_cost").await;

    match result {
        Ok(mut response) => match response.take::<Vec<SmsCost>>(0) {
            Ok(sms_costs) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "SMS Costs retrieved successfully".to_string(),
                data: Some(sms_costs),
            }),
            Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: format!("Failed to process SMS costs: {}", e),
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
pub async fn update_sms_cost(
    db: web::Data<Surreal<Client>>,
    sms_id: web::Path<String>,
    costs: web::Json<CreateSmsCost>,
) -> impl Responder {
    let thing = Thing::from(("sms_cost", sms_id.as_str()));
    let provider = costs.provider.clone();
    let cost = costs.cost.clone();
    let currency = costs.currency.clone();

    let sql = r#"
        UPDATE $thing 
        SET 
            provider = $provider, 
            cost = $cost,
            currency = $currency,
            updated_at = time::now()
    "#;

    let result = db
        .query(sql)
        .bind(("thing", thing))
        .bind(("provider", provider))
        .bind(("cost", cost))
        .bind(("currency", currency))
        .await;

    match result {
        Ok(mut response) => match response.take::<Option<SmsCost>>(0) {
            Ok(Some(cost)) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "SMS cost updated successfully".to_string(),
                data: Some(cost),
            }),
            Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "SMS cost not found".to_string(),
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

pub async fn delete_sms_cost(
    db: web::Data<Surreal<Client>>,
    sms_id: web::Path<String>,
) -> impl Responder {
    let thing = Thing::from(("sms_cost", sms_id.as_str()));

    let result = db.query("DELETE $thing").bind(("thing", thing)).await;

    match result {
        Ok(mut response) => match response.take::<Vec<SmsCost>>(0) {
            Ok(deleted_sms_cost) => {
                if deleted_sms_cost.is_empty() {
                    HttpResponse::Ok().json(ApiResponse::<()> {
                        status: "success".to_string(),
                        message: "SMS cost deleted successfully".to_string(),
                        data: None,
                    })
                } else {
                    HttpResponse::Ok().json(ApiResponse::<()> {
                        status: "success".to_string(),
                        message: "SMS cost deleted successfully".to_string(),
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
