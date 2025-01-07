use crate::models::admin::{Admin, CreateAdmin};
use crate::models::response::ApiResponse;
use actix_web::{web, HttpResponse, Responder};
//use chrono::{DateTime, Utc};
use surrealdb::engine::remote::ws::Client;
//use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub async fn create_admin(
    db: web::Data<Surreal<Client>>,
    admin: web::Json<CreateAdmin>,
) -> impl Responder {
    let username = admin.username.clone();
    let email = admin.email.clone();
    let hashed_password = bcrypt::hash(&admin.password, 10).unwrap();
    let role = admin.role.clone();
    let telepon = admin.telepon.clone();
    let alamat = admin.alamat.clone();

    let sql = r#"
        CREATE admin 
        SET 
            username = $username, 
            email = $email, 
            password = $password,
            role = $role,
            telepon = $telepon,
            alamat = $alamat,
            created_at = time::now(), 
            updated_at = time::now()
    "#;

    let result = db
        .query(sql)
        .bind(("username", username))
        .bind(("email", email))
        .bind(("password", hashed_password))
        .bind(("role", role))
        .bind(("telepon", telepon))
        .bind(("alamat", alamat))
        .await;

    match result {
        Ok(mut response) => match response.take::<Option<Admin>>(0) {
            Ok(Some(admin)) => HttpResponse::Ok().json(ApiResponse {
                status: "success".to_string(),
                message: "User created successfully".to_string(),
                data: Some(admin),
            }),
            Ok(None) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "Failed to create user".to_string(),
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

pub async fn get_admin(db: web::Data<Surreal<Client>>) -> impl Responder {
    log::debug!("Attempting to get all admin");
    let result = db.query("SELECT * FROM admin").await;

    match result {
        Ok(mut response) => {
            log::debug!("Query successful, processing response");
            match response.take::<Vec<Admin>>(0) {
                Ok(admin) => {
                    log::debug!("Found {} users", admin.len());
                    HttpResponse::Ok().json(ApiResponse {
                        status: "success".to_string(),
                        message: "Users retrieved successfully".to_string(),
                        data: Some(admin),
                    })
                }
                Err(e) => {
                    log::error!("Error taking users from response: {}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        status: "error".to_string(),
                        message: format!("Failed to process users: {}", e),
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
