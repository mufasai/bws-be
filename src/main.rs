use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{guard, middleware::Logger, web, App, HttpResponse, HttpServer};
use env_logger::Env;
use handlers::inbox::get_inbox;
use handlers::input_sms::{input_sms_, upload_json_file};
use handlers::sms::{generate_otp};
mod db;
mod handlers;
mod models;
mod routes;
use crate::handlers::auth::register;

// Fungsi handler untuk request OPTIONS (preflight request)
async fn options_handler() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "http://localhost:5173"))
        .insert_header((
            header::ACCESS_CONTROL_ALLOW_METHODS,
            "GET, POST, PUT, DELETE, OPTIONS",
        ))
        .insert_header((
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            "Content-Type, Authorization",
        ))
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
                    .allowed_origin("http://localhost:5173")
                    .allowed_origin("192.168.1.15")
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION])
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
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
                web::resource("/api/sms/generate")
                    .guard(guard::Post())
                    .to(generate_otp),
            )
            .service(
                web::resource("/api/inbox")
                    .guard(guard::Get())
                    .to(get_inbox),
            )
            .service(
                web::resource("/api/sms/input")
                    .guard(guard::Post())
                    .to(input_sms_),
            )
            .service(
                web::resource("/api/sms/upload")
                    .guard(guard::Post())
                    .to(upload_json_file),
            )

            // Konfigurasi route lainnya
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")? // Bind server ke localhost
    .run()
    .await
}
