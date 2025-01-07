use actix_web::{web, HttpResponse, Responder};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use chrono::{DateTime, Utc};
use crate::models::user::{User, CreateUser, UserGet , FinishingUser};
use crate::models::response::ApiResponse;

pub async fn create_user(
    db: web::Data<Surreal<Client>>,
    user: web::Json<CreateUser>,
) -> impl Responder {
    let username = user.username.clone();
    let email = user.email.clone();
    let hashed_password = bcrypt::hash(&user.password, 10).unwrap();
    let phone_number = user.phone_number.clone();
    let role = Thing::from(("groups", user.role.as_str()));
    let fullname = user.fullname.clone();
    let address = user.address.clone();
    let country = user.country.clone();
    let city = user.city.clone();
    let state = user.state.clone();
    let country_code = user.country_code.clone();
    let verification_status = user.verification_status.clone();
    
    let sql = r#"
    CREATE users 
    SET 
        username = $username, 
        email = $email, 
        password = $password, 
        phone_number = $phone_number,
        role = $role,
        fullname = $fullname,
        address = $address,
        country = $country,
        city = $city,
        state = $state,
        country_code = $country_code,
        verification_status = $verification_status,
        created_at = time::now(), 
        updated_at = time::now()
    "#;
    
    let result = db
        .query(sql)
        .bind(("username", username))
        .bind(("email", email))
        .bind(("password", hashed_password))
        .bind(("phone_number", phone_number))
        .bind(("role", role))
        .bind(("fullname", fullname))
        .bind(("address", address))
        .bind(("country", country))
        .bind(("city", city))
        .bind(("state", state))
        .bind(("country_code", country_code))
        .bind(("verification_status", verification_status))
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
    let result = db.query("SELECT * , role.role_name , role.role_description ,role.id  FROM users").await;
    
    match result {
        Ok(mut response) => {
            log::debug!("Query successful, processing response");
            match response.take::<Vec<UserGet>>(0) {
                Ok(users) => {
                    
                    log::debug!("Found {} users", users.len());

                    let transformed_users: Vec<FinishingUser> = users
                    .into_iter()
                    .map(|user| { 

                        // let role_id = user.role.id;
                        // println!("Role ID: {:?}", role_id);

                        FinishingUser {
                        id: user.id,
                        username: user.username,
                        email: user.email,
                        password: user.password,
                        phone_number: user.phone_number,
                        role_id: user.role.id,
                        role_name: user.role.role_name,
                        role_description: user.role.role_description,
                        fullname: user.fullname,
                        address: user.address,
                        country: user.country,
                        city: user.city,
                        state: user.state,
                        country_code: user.country_code,
                        verification_status: user.verification_status,
                        created_at: user.created_at,
                        updated_at: user.updated_at,
                        }
                    }).collect();

                    HttpResponse::Ok().json(ApiResponse {
                        status: "success".to_string(),
                        message: "Users retrieved successfully".to_string(),
                        data: Some(transformed_users),
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
    let user_id_str = user_id.as_str();
    let thing = Thing::from(("users", user_id.as_str()));
    let username = user.username.clone();
    let email = user.email.clone();
    let hashed_password = bcrypt::hash(&user.password, 10).unwrap();
    let phone_number = user.phone_number.clone();
    let role = Thing::from(("groups", user.role.as_str()));
    let fullname = user.fullname.clone();
    let address = user.address.clone();
    let country = user.country.clone();
    let city = user.city.clone();
    let state = user.state.clone();
    let country_code = user.country_code.clone();
    let verification_status = user.verification_status.clone();
    
    let sql = r#"
    UPDATE $thing 
    SET 
        username = $username, 
        email = $email, 
        password = $password, 
        phone_number = $phone_number,
        role = $role,
        fullname = $fullname,
        address = $address,
        country = $country,
        city = $city,
        state = $state,
        country_code = $country_code,
        verification_status = $verification_status,
        updated_at = time::now()
    "#;
    
    let result = db
        .query(sql)
        .bind(("thing", thing))
        .bind(("username", username))
        .bind(("email", email))
        .bind(("password", hashed_password))
        .bind(("phone_number", phone_number))
        .bind(("role", role))
        .bind(("fullname", fullname))
        .bind(("address", address))
        .bind(("country", country))
        .bind(("city", city))
        .bind(("state", state))
        .bind(("country_code", country_code))
        .bind(("verification_status", verification_status))
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