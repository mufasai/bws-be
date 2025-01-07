use actix_web::{web, HttpResponse, Responder};
use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use crate::models::user::{LoginRequest, LoginResponse, User};
use crate::models::response::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn login(
    db: web::Data<Surreal<Client>>,
    login_req: web::Json<LoginRequest>,
) -> impl Responder {
    let username = login_req.username.clone();
    let password = login_req.password.clone();
    
    let result = db
        .query("SELECT * FROM users WHERE username = $username")
        .bind(("username", username))
        .await;

    match result {
        Ok(mut response) => {
            match response.take::<Vec<User>>(0) {
                Ok(users) => {
                    if let Some(user) = users.first() {
                        if verify(&password, &user.password).unwrap_or(false) {
                            let claims = Claims {
                                sub: user.username.clone(),
                                exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
                            };

                            let token = encode(
                                &Header::default(),
                                &claims,
                                &EncodingKey::from_secret("your_secret_key".as_ref()),
                            )
                            .unwrap();

                            HttpResponse::Ok().json(ApiResponse {
                                status: "success".to_string(),
                                message: "Login successful".to_string(),
                                data: Some(LoginResponse { token }),
                            })
                        } else {
                            HttpResponse::Unauthorized().json(ApiResponse::<()> {
                                status: "error".to_string(),
                                message: "Invalid credentials".to_string(),
                                data: None,
                            })
                        }
                    } else {
                        HttpResponse::Unauthorized().json(ApiResponse::<()> {
                            status: "error".to_string(),
                            message: "User not found".to_string(),
                            data: None,
                        })
                    }
                }
                Err(e) => {
                    log::error!("Error processing login response: {}", e);
                    HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        status: "error".to_string(),
                        message: e.to_string(),
                        data: None,
                    })
                }
            }
        }
        Err(e) => {
            log::error!("Database error in login: {}", e);
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: e.to_string(),
                data: None,
            })
        }
    }
} 