use crate::handlers::*;

use axum::{
    routing::get,
    Router,
    Extension,
};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt
};
use dotenv;
use tower_http::cors::{Any, CorsLayer};

use sqlx::postgres::PgPoolOptions;

mod handlers;
mod templates;
mod types;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Read environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialise logging
    let log_level = std::env::var("RUST_LOG")
        .expect("Should have RUST_LOG as an environment variable");
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Set up basic CORS
    let cors = CorsLayer::new().allow_origin(Any);

    // Set up the Postgres db
    let db_url = std::env::var("DB_URL").expect("Need db url");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Initialise the axum web server app
    let app = Router::new()
        .route(
            "/",
            get(home),
        )
        .route(
            "/profile",
            get(profile)
        )
        .layer(cors)
        .layer(Extension(pool));

    let addr = "0.0.0.0:3000".parse()?;

    tracing::debug!("Listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
