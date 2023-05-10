use axum::{
    response::Html,
    routing::{
        get,
        post
    },
    Router,
    extract::Query,
};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt
};
use serde::{Serialize, Deserialize};
use dotenv;

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

    // Initialise the axum web server app
    let app = Router::new()
        .route(
            "/",
            get(|| async { "Hello world" }),
        );

    let addr = "0.0.0.0:3000".parse()?;

    tracing::debug!("Listening on {}", &addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
