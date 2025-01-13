use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, middleware::Logger, guard};
use actix_web::http::header;
use env_logger::Env;

mod db;
mod routes;
mod models;
mod handlers;

use crate::handlers::auth::register;
use crate::handlers::input_sms_direct;
use crate::input_sms_direct::input_sms_direct;


// Fungsi handler untuk request OPTIONS (preflight request)
async fn options_handler() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173"))
        .insert_header((header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, PUT, DELETE, OPTIONS"))
        .insert_header((header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization"))
        .insert_header((header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true"))
        .insert_header((header::ACCESS_CONTROL_MAX_AGE, "3600"))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inisialisasi logger
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    // Inisialisasi database (contoh, pastikan Anda menyesuaikannya dengan kode Anda)
    let db = db::initialize_db()
        .await
        .expect("Failed to connect to database");
    let db = web::Data::new(db);

    println!("Server running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            // Tambahkan logger middleware
            .wrap(Logger::default())
            // Konfigurasi middleware CORS
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173") // Ganti sesuai URL frontend Anda
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600)
            )
            
            // Tambahkan route untuk register dengan metode POST
            .service(
                web::resource("/api/auth/register")
                    .guard(guard::Post())
                    .to(register),
            )
            // Tambahkan handler untuk OPTIONS pada endpoint register
            .service(
                web::resource("/api/auth/register")
                    .guard(guard::Options())
                    .to(options_handler),
            )

            .service(
                web::scope("/api/sms")
                    .service(
                        web::resource("/input")
                            .guard(guard::Post())
                            .to(input_sms_direct)
                    )
                    .service(
                        web::resource("/input")
                            .guard(guard::Options())
                            .to(options_handler)
                    )
            )

            // Konfigurasi route lainnya
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")? // Bind server ke localhost
    .run()
    .await
}
