use crate::models::response::ApiResponse;
use crate::models::user::{
    self, ForgotPasswordRequest, ForgotPasswordResponse, LoginRequest, LoginResponse, RegUser,
    ResetPasswordRequest, UserLogreg, VerifyEmailRequest,
};
use actix_web::{web, HttpResponse};
use bcrypt::{hash, verify};
use jsonwebtoken::{encode, EncodingKey, Header};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Write;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

#[derive(Debug, Serialize)]
struct UserResponse {
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    status: String,
    verification_code: String,
    is_verified: bool,
    dob: Option<String>, // Menggunakan Option<String> untuk dob agar dapat menangani nilai null
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

const SMTP_USERNAME: &str = "fauzilhokki@gmail.com";
const SMTP_PASSWORD: &str = "aitdtoqhcfimxojg";
const JWT_SECRET: &str = "12345678";

pub async fn get_user(db: web::Data<Surreal<Client>>) -> HttpResponse {
    let result = db.query("SELECT * FROM users").await;

    match result {
        Ok(mut response) => {
            match response.take::<Vec<UserLogreg>>(0) {
                Ok(users) => {
                    if !users.is_empty() {
                        let user_responses: Vec<UserResponse> = users
                            .into_iter()
                            .map(|user| UserResponse {
                                username: user.username,
                                first_name: user.first_name,
                                last_name: user.last_name,
                                email: user.email,
                                status: if user.is_verified {
                                    "verified".to_string()
                                } else {
                                    "pending".to_string()
                                },
                                verification_code: user.verification_code.clone(),
                                is_verified: user.is_verified,
                                dob: user.dob.map(|date| date.to_string()), // Mengonversi NaiveDate ke String jika ada
                            })
                            .collect();

                        HttpResponse::Ok().json(ApiResponse {
                            status: "success".to_string(),
                            message: "Users retrieved successfully".to_string(),
                            data: Some(user_responses),
                        })
                    } else {
                        HttpResponse::NotFound().json(ApiResponse::<()> {
                            status: "error".to_string(),
                            message: "No users found".to_string(),
                            data: None,
                        })
                    }
                }
                Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: format!("Error processing users: {}", e),
                    data: None,
                }),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: format!("Database error: {}", e),
            data: None,
        }),
    }
}

fn send_verification_email(to_email: &str, token: &str) -> Result<(), String> {
    let mut html_content = String::new();

    // Menulis HTML ke dalam String
    write!(
        &mut html_content,
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {{
                    font-family: Arial, sans-serif;
                    background-color: #f4f4f4;
                    margin: 0;
                    padding: 0;
                }}
                .email-container {{
                    max-width: 600px;
                    margin: 20px auto;
                    background-color: #ffffff;
                    padding: 20px;
                    border-radius: 8px;
                    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
                }}
                .header {{
                    text-align: center;
                    padding: 10px 0;
                    border-bottom: 1px solid #ddd;
                }}
                .header h2 {{
                    color: #333;
                }}
                .content {{
                    padding: 20px 0;
                    text-align: center;
                }}
                .content p {{
                    font-size: 16px;
                    color: #555;
                }}
                .token-box {{
                    display: inline-block;
                    padding: 10px 20px;
                    font-size: 18px;
                    font-weight: bold;
                    color: #ffffff;
                    background-color: #4CAF50;
                    border-radius: 5px;
                    margin: 10px 0;
                }}
                .footer {{
                    margin-top: 20px;
                    font-size: 12px;
                    color: #999;
                    text-align: center;
                }}
            </style>
        </head>
        <body>
            <div class="email-container">
                <div class="header">
                    <h2>Verify Your Email Address</h2>
                </div>
                <div class="content">
                    <p>Thank you for registering! Use the verification code below to verify your email address:</p>
                    <div class="token-box">{}</div>
                </div>
                <div class="footer">
                    <p>If you did not request this, please ignore this email.</p>
                </div>
            </div>
        </body>
        </html>
        "#,
        token
    ).map_err(|e| e.to_string())?;

    let email = Message::builder()
        .from(SMTP_USERNAME.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject("Verify Your Email Address")
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(html_content)
        .map_err(|e| e.to_string())?;

    let creds = Credentials::new(SMTP_USERNAME.to_string(), SMTP_PASSWORD.to_string());

    // Menambahkan waktu timeout pada SMTP connection
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .map_err(|e| e.to_string())?
        .credentials(creds)
        .timeout(Some(std::time::Duration::new(10, 0))) // Timeout 10 detik
        .build();

    // Mengirim email
    mailer.send(&email).map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn register(db: web::Data<Surreal<Client>>, user: web::Json<RegUser>) -> HttpResponse {
    let user_data = user.into_inner();

    // Create owned String values
    let email = user_data.email.clone();
    let username = user_data.username.clone();
    let first_name = user_data.first_name.clone();
    let last_name = user_data.last_name.clone();

    // Convert the query parameters into owned Strings that will live for the duration of the function
    let query = format!("SELECT * FROM users WHERE email = $email OR username = $username");

    // Check existing user
    let existing_user = db
        .query(query)
        .bind(("email", email.clone())) // Use owned values
        .bind(("username", username.clone())) // Use owned values
        .await;

    match existing_user {
        Ok(mut response) => {
            if let Ok(users) = response.take::<Vec<UserLogreg>>(0) {
                if !users.is_empty() {
                    return HttpResponse::BadRequest().json(ApiResponse::<()> {
                        status: "error".to_string(),
                        message: "Email or username already exists".to_string(),
                        data: None,
                    });
                }
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: format!("Database error: {}", e),
                data: None,
            })
        }
    }

    // Generate verification code as owned String
    let verification_code = format!("{:06}", rand::thread_rng().gen_range(0..1000000));

    // Hash password
    let hashed_password = match hash(&user_data.password, 10) {
        Ok(hashed) => hashed,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: format!("Password hashing error: {}", e),
                data: None,
            })
        }
    };

    // Create the query string
    let create_query = format!(
        r#"
        CREATE users SET 
        username = $username,
        first_name = $first_name,
        last_name = $last_name,
        email = $email,
        password = $password,
        verification_code = $verification_code,
        is_verified = false,
        dob = $dob
        "#
    );
    

    // Create user in database using owned values
    let create_result = db
        .query(create_query)
        .bind(("username", username.clone()))
        .bind(("first_name", first_name))
        .bind(("last_name", last_name))
        .bind(("email", email.clone()))
        .bind(("password", hashed_password))
        .bind(("verification_code", verification_code.clone()))
        .bind(("dob", user_data.dob))
        .await;

    match create_result {
        Ok(_) => {
            // Send verification email
            match send_verification_email(&email, &verification_code) {
                Ok(_) => HttpResponse::Ok().json(ApiResponse {
                    status: "success".to_string(),
                    message: "Registration successful. Please check your email for verification."
                        .to_string(),
                    data: Some(json!({
                        "email": email,
                        "verification_code": verification_code
                    })),
                }),
                Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: format!("Failed to send verification email: {}", e),
                    data: None,
                }),
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: format!("Database error: {}", e),
            data: None,
        }),
    }
}

pub async fn verify_email(
    db: web::Data<Surreal<Client>>,
    query: web::Query<VerifyEmailRequest>,
) -> HttpResponse {
    println!("Full Query: {:?}", query); // Log entire query

    if query.token.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "Token is empty"
        }));
    }

    // Mencari pengguna berdasarkan verification_code
    let query_result = db
        .query("SELECT * FROM users WHERE verification_code = $verification_code")
        .bind(("verification_code", query.token.clone()))
        .await;

    match query_result {
        Ok(mut response) => {
            match response.take::<Vec<UserLogreg>>(0) {
                Ok(users) => {
                    if let Some(user) = users.first() {
                        // Jika pengguna ditemukan, perbarui status is_verified menjadi true
                        let update_result = db
                            .query("UPDATE users SET is_verified = true WHERE verification_code = $verification_code")
                            .bind(("verification_code", query.token.clone()))
                            .await;

                        match update_result {
                            Ok(_) => {
                                return HttpResponse::Ok().json(serde_json::json!( {
                                    "success": true,
                                    "message": "Email verification successful"
                                }));
                            }
                            Err(e) => {
                                return HttpResponse::InternalServerError().json(
                                    ApiResponse::<()> {
                                        status: "error".to_string(),
                                        message: format!(
                                            "Error updating verification status: {}",
                                            e
                                        ),
                                        data: None,
                                    },
                                );
                            }
                        }
                    } else {
                        return HttpResponse::BadRequest().json(serde_json::json!( {
                            "success": false,
                            "message": "User not found"
                        }));
                    }
                }
                Err(e) => {
                    return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                        status: "error".to_string(),
                        message: format!("Error checking verification code: {}", e),
                        data: None,
                    });
                }
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: format!("Database error: {}", e),
                data: None,
            });
        }
    }
}

pub(crate) async fn login(
    db: web::Data<Surreal<Client>>,
    login_req: web::Json<LoginRequest>,
) -> HttpResponse {
    let login_req = login_req.into_inner();
    let username = login_req.username.clone();
    let password = login_req.password.clone();

    let result = db
        .query("SELECT * FROM users WHERE username = $username")
        .bind(("username", username))
        .await;

    match result {
        Ok(mut response) => {
            if let Ok(users) = response.take::<Vec<UserLogreg>>(0) {
                if let Some(user) = users.first() {
                    if !user.is_verified {
                        return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                            status: "error".to_string(),
                            message: "Email not verified".to_string(),
                            data: None,
                        });
                    }

                    println!("user masuk -> {:#?}", user);
                    if verify(&password, &user.password).unwrap_or(false) {
                        let claims = Claims {
                            sub: user.username.clone(),
                            exp: (std::time::SystemTime::now()
                                + std::time::Duration::from_secs(3600))
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_secs() as usize,
                        };

                        let token = match encode(
                            &Header::default(),
                            &claims,
                            &EncodingKey::from_secret(JWT_SECRET.as_ref()),
                        ) {
                            Ok(t) => t,
                            Err(_) => {
                                return HttpResponse::InternalServerError().json(
                                    ApiResponse::<()> {
                                        status: "error".to_string(),
                                        message: "Failed to create JWT token".to_string(),
                                        data: None,
                                    },
                                );
                            }
                        };

                        println!("response login ->{:#?} ", response);
                        return HttpResponse::Ok().json(LoginResponse {
                            message: "Login successful".to_string(),
                            token,
                            id : user.id.clone()
                           
                        });
                    } else {
                        return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                            status: "error".to_string(),
                            message: "Incorrect password".to_string(),
                            data: None,
                        });
                    }
                }
            }

            HttpResponse::NotFound().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "User not found".to_string(),
                data: None,
            })
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: "Internal server error".to_string(),
                data: None,
            })
        }
    }
}

pub async fn forgot_password(
    db: web::Data<Surreal<Client>>,
    req: web::Json<ForgotPasswordRequest>,
) -> HttpResponse {
    let email = req.email.clone();

    // Cek apakah email terdaftar
    let result = db
        .query("SELECT * FROM users WHERE email = $email")
        .bind(("email", email.clone()))
        .await;

    match result {
        Ok(mut response) => {
            if let Ok(users) = response.take::<Vec<UserLogreg>>(0) {
                if let Some(user) = users.first() {
                    // Generate verification code baru
                    let verification_code =
                        format!("{:06}", rand::thread_rng().gen_range(0..1000000));

                    // Update verification code di database
                    let update_result = db
                        .query("UPDATE users SET verification_code = $verification_code WHERE email = $email")
                        .bind(("verification_code", verification_code.clone()))
                        .bind(("email", email.clone()))
                        .await;

                    match update_result {
                        Ok(_) => {
                            // Kirim email verifikasi
                            match send_verification_email(&email, &verification_code) {
                                Ok(_) => HttpResponse::Ok().json(ForgotPasswordResponse {
                                    message: "Verification code sent to your email".to_string(),
                                    email: email,
                                }),
                                Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<
                                    (),
                                > {
                                    status: "error".to_string(),
                                    message: format!("Failed to send verification email: {}", e),
                                    data: None,
                                }),
                            }
                        }
                        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                            status: "error".to_string(),
                            message: format!("Database error: {}", e),
                            data: None,
                        }),
                    }
                } else {
                    HttpResponse::NotFound().json(ApiResponse::<()> {
                        status: "error".to_string(),
                        message: "Email not found".to_string(),
                        data: None,
                    })
                }
            } else {
                HttpResponse::NotFound().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: "Email not found".to_string(),
                    data: None,
                })
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: format!("Database error: {}", e),
            data: None,
        }),
    }
}

pub async fn reset_password(
    db: web::Data<Surreal<Client>>,
    req: web::Json<ResetPasswordRequest>,
) -> HttpResponse {
    let email = req.email.clone();
    let verification_code = req.verification_code.clone();
    let new_password = req.new_password.clone();

    // Verifikasi code
    let verify_result = db
        .query("SELECT * FROM users WHERE email = $email AND verification_code = $verification_code")
        .bind(("email", email.clone()))
        .bind(("verification_code", verification_code.clone()))
        .await;

    match verify_result {
        Ok(mut response) => {
            if let Ok(users) = response.take::<Vec<UserLogreg>>(0) {
                if let Some(_) = users.first() {
                    // Hash password baru
                    let hashed_password = match hash(&new_password, 10) {
                        Ok(hashed) => hashed,
                        Err(e) => {
                            return HttpResponse::InternalServerError().json(ApiResponse::<()> {
                                status: "error".to_string(),
                                message: format!("Password hashing error: {}", e),
                                data: None,
                            })
                        }
                    };

                    // Update password
                    let update_result = db
                        .query("UPDATE users SET password = $password, verification_code = '' WHERE email = $email")
                        .bind(("password", hashed_password))
                        .bind(("email", email))
                        .await;

                    match update_result {
                        Ok(_) => HttpResponse::Ok().json(ApiResponse {
                            status: "success".to_string(),
                            message: "Password successfully reset".to_string(),
                            data: None as Option<()>,
                        }),
                        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
                            status: "error".to_string(),
                            message: format!("Database error: {}", e),
                            data: None,
                        }),
                    }
                } else {
                    HttpResponse::BadRequest().json(ApiResponse::<()> {
                        status: "error".to_string(),
                        message: "Invalid verification code".to_string(),
                        data: None,
                    })
                }
            } else {
                HttpResponse::BadRequest().json(ApiResponse::<()> {
                    status: "error".to_string(),
                    message: "Invalid verification code".to_string(),
                    data: None,
                })
            }
        }
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: format!("Database error: {}", e),
            data: None,
        }),
    }
}
