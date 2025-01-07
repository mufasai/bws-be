use actix_web::web;
use crate::handlers::{users, auth, dashboard, groups};

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
                web::scope("/groups")
                    .route("", web::get().to(groups::get_groups))
                    .route("", web::post().to(groups::create_group))
                    .route("/{id}", web::put().to(groups::update_group))
                    .route("/{id}", web::delete().to(groups::delete_group))
            )
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(auth::login))
            )
            .service(
                web::scope("/dashboard")
                    .route("/summary", web::get().to(dashboard::get_summary))
            )
    );
} 