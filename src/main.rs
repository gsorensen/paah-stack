use axum::{
    response::Html,
    routing::get,
    Router,
    Extension,
    extract::Query,
};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt
};
use serde::{Serialize, Deserialize};
use dotenv;
use askama::Template;
use tower_http::cors::{Any, CorsLayer};

use sqlx::postgres::PgPoolOptions;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn home() -> Html<String> {
    let r = IndexTemplate {};

    let template = match r.render() {
        Ok(rendered_template) => rendered_template,
        Err(err) => {
            let err_msg = format!("Failed to render index.html with following error: {}", err);
            tracing::error!(err_msg);
            err_msg
        }
    };

    tracing::debug!("Serving index.html");
    Html(template)
}

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    name: String,
}

#[derive(Template)]
#[template(path = "pages/profile.html")]
struct ProfileTemplate {
    profile: Profile,
}

async fn profile(Query(profile): Query<Profile>) -> Html<String> {
    tracing::debug!("Fetching {:?}", profile);

    if profile.name.is_empty() {
        return Html("".into());
    }

    let r = ProfileTemplate { profile };

    let template = match r.render() {
        Ok(rendered_template) => rendered_template,
        Err(err) => {
            let err_msg = format!("Failed to render profile with following error: {}", err);
            tracing::error!(err_msg);
            err_msg
        }
    };

    tracing::debug!("Serving profile");
    Html(template)
}

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
