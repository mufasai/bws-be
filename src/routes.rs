use actix_web::web;
use crate::handlers::{users, auth, dashboard};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/users")
                    .route("", web::get().to(users::get_users))
                    .route("", web::post().to(users::create_user))
                    .route("/{id}", web::put().to(users::update_user))
                    .route("/{id}", web::delete().to(users::delete_user))
            )
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(auth::login))
                    .route("/register", web::post().to(auth::register))
                    .route("/get_user", web::get().to(auth::get_user))
                    .route("/verify_email", web::get().to(auth::verify_email))  
                    .route("/forgot_password", web::post().to(auth::forgot_password)) // Forgot Password
                    .route("/reset_password", web::post().to(auth::reset_password)), // Reset Password

            )
            .service(
                web::scope("/dashboard")
                    .route("/summary", web::get().to(dashboard::get_summary))
            )
    );
}