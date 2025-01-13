mod db;
mod routes;
mod models;
mod handlers;

use actix_web::{web, App, HttpServer, middleware::Logger};
use env_logger::Env;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let db = db::initialize_db()
        .await
        .expect("Failed to connect to database");
    
    let db = web::Data::new(db);

    println!("Server running at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173") 
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        actix_web::http::header::CONTENT_TYPE,
                        actix_web::http::header::AUTHORIZATION,
                    ])
                    .supports_credentials()
            )
            .app_data(db.clone())
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
