use actix_web::{web, HttpResponse, Responder};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use crate::models::user::{User, CreateUser};
use crate::models::response::ApiResponse;

pub async fn create_user(
    db: web::Data<Surreal<Client>>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    let username = user.username.clone();
    let email = user.email.clone();
    let hashed_password = bcrypt::hash(&user.password, 10).unwrap();
    
    let sql = r#"
        CREATE users 
        SET 
            username = $username, 
            email = $email, 
            password = $password, 
            created_at = time::now(), 
            updated_at = time::now()
    "#;
    
    let result = db
        .query(sql)
        .bind(("username", username))
        .bind(("email", email))
        .bind(("password", hashed_password))
        .await;

    match result {
        Ok(mut response) => {
            match response.take::<Option<User>>(0) {
                Ok(Some(user)) => HttpResponse::Ok().json(ApiResponse {
                    status: "success".to_string(),
                    message: "User created successfully".to_string(),
                    data: Some(user),
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
            }
        }
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

pub async fn get_users(db: web::Data<Surreal<Client>>) -> impl Responder {
    log::debug!("Attempting to get all users");
    let result = db.query("SELECT * FROM users").await;
    
    match result {
        Ok(mut response) => {
            log::debug!("Query successful, processing response");
            match response.take::<Vec<User>>(0) {
                Ok(users) => {
                    log::debug!("Found {} users", users.len());
                    HttpResponse::Ok().json(ApiResponse {
                        status: "success".to_string(),
                        message: "Users retrieved successfully".to_string(),
                        data: Some(users),
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

pub async fn update_user(
    db: web::Data<Surreal<Client>>,
    user_id: web::Path<String>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    let thing = Thing::from(("users", user_id.as_str()));
    let username = user.username.clone();
    let email = user.email.clone();
    
    let sql = r#"
        UPDATE $thing 
        SET 
            username = $username, 
            email = $email, 
            updated_at = time::now()
    "#;
    
    let result = db
        .query(sql)
        .bind(("thing", thing))
        .bind(("username", username))
        .bind(("email", email))
        .await;

    match result {
        Ok(mut response) => {
            match response.take::<Option<User>>(0) {
                Ok(Some(user)) => HttpResponse::Ok().json(ApiResponse {
                    status: "success".to_string(),
                    message: "User updated successfully".to_string(),
                    data: Some(user),
                }),
                Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: "User not found".to_string(),
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

pub async fn delete_user(
    db: web::Data<Surreal<Client>>,
    user_id: web::Path<String>,
) -> impl Responder {
    let thing = Thing::from(("users", user_id.as_str()));
    
    let result = db
        .query("DELETE $thing")
        .bind(("thing", thing))
        .await;

    match result {
        Ok(mut response) => {
            match response.take::<Vec<User>>(0) {
                Ok(deleted_users) => {
                    if deleted_users.is_empty() {
                        HttpResponse::Ok().json(ApiResponse::<()> {
                            status: "success".to_string(),
                            message: "User deleted successfully".to_string(),
                            data: None,
                        })
                    } else {
                        HttpResponse::Ok().json(ApiResponse::<()> {
                            status: "success".to_string(),
                            message: "User deleted successfully".to_string(),
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