use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub async fn initialize_db() -> Result<Surreal<Client>, surrealdb::Error> {
    log::info!("Connecting to database...");
    let db = Surreal::new::<Ws>("127.0.0.1:8001").await?;

    log::info!("Signing in to database...");
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    log::info!("Selecting namespace and database...");
    db.use_ns("bws").use_db("bws_otp").await?;

    log::info!("Database connection established successfully");
    Ok(db)
}
