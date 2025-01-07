use actix_web::{web, HttpResponse, Responder};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use crate::models::group::{Group, CreateGroup};
use crate::models::response::ApiResponse;
use surrealdb::sql::Thing;

pub async fn create_group(
    db: web::Data<Surreal<Client>>,
    group: web::Json<CreateGroup>,
) -> impl Responder {
    let role_name = group.role_name.clone();
    let role_description = group.role_description.clone();

    let sql = r#"
    CREATE groups 
    SET 
        role_name = $role_name, 
        role_description = $role_description
    "#;

    let result = db
        .query(sql)
        .bind(("role_name", role_name))
        .bind(("role_description", role_description))
        .await;

    match result {
        Ok(mut response) => {
            match response.take::<Option<Group>>(0) {
                Ok(Some(group)) => HttpResponse::Ok().json(ApiResponse {
                    status: "success".to_string(),
                    message: "Group created successfully".to_string(),
                    data: Some(group),
                }),
                Ok(None) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: "Failed to create group".to_string(),
                    data: None,
                }),
                Err(e) => {
                    log::error!("Error processing create group response: {}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        status: "error".to_string(),
                        message: e.to_string(),
                        data: None,
                    })
                }
            }
        }
        Err(e) => {
            log::error!("Database error in create group: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: e.to_string(),
                data: None,
            })
        }
    }
}

pub async fn get_groups(db: web::Data<Surreal<Client>>) -> impl Responder {
    log::debug!("Attempting to get all groups");
    let result = db.query("SELECT * FROM groups").await;
    
    match result {
        Ok(mut response) => {
            log::debug!("Query successful, processing response");
            match response.take::<Vec<Group>>(0) {
                Ok(groups) => {
                    log::debug!("Found {} groups", groups.len());
                    HttpResponse::Ok().json(ApiResponse {
                        status: "success".to_string(),
                        message: "Groups retrieved successfully".to_string(),
                        data: Some(groups),
                    })
                }
                Err(e) => {
                    log::error!("Error taking groups from response: {}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        status: "error".to_string(),
                        message: format!("Failed to process groups: {}", e),
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

pub async fn update_group(
    db: web::Data<Surreal<Client>>,
    group_id: web::Path<String>,
    group: web::Json<CreateGroup>,
) -> impl Responder {
    let group_id_str = group_id.as_str();
    let thing = Thing::from(("groups", group_id.as_str()));
    let role_name = group.role_name.clone();
    let role_description = group.role_description.clone();
    
    let sql = r#"
    UPDATE $thing 
    SET 
        role_name = $role_name,
        role_description = $role_description
    "#;
    
    let result = db
        .query(sql)
        .bind(("thing", thing))
        .bind(("role_name", role_name))
        .bind(("role_description", role_description))
        .await;

    match result {
        Ok(mut response) => {
            match response.take::<Option<Group>>(0) {
                Ok(Some(group)) => HttpResponse::Ok().json(ApiResponse {
                    status: "success".to_string(),
                    message: "Group updated successfully".to_string(),
                    data: Some(group),
                }),
                Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: "Group not found".to_string(),
                    data: None,
                }),
                Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: e.to_string(),
                    data: None,
                }),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: e.to_string(),
            data: None,
        }),
    }
}

pub async fn delete_group(
    db: web::Data<Surreal<Client>>,
    group_id: web::Path<String>,
) -> impl Responder {
    let thing = Thing::from(("groups", group_id.as_str()));

    let result = db
        .query("DELETE $thing")
        .bind(("thing", thing))
        .await;

    match result {
        Ok(mut response) => {
            match response.take::<Vec<Group>>(0) {
                Ok(deleted_groups) => {
                    if deleted_groups.is_empty() {
                        HttpResponse::Ok().json(ApiResponse::<()> {
                            status: "success".to_string(),
                            message: "Group deleted successfully".to_string(),
                            data: None,
                        })
                    } else {
                        HttpResponse::Ok().json(ApiResponse::<()> {
                            status: "success".to_string(),
                            message: "Group deleted successfully".to_string(),
                            data: None,
                        })
                    }
                }
                Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: e.to_string(),
                    data: None,
                }),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: e.to_string(),
            data: None,
        }),
    }
}