use axum::{
    response::Html,
    extract::Query, http::StatusCode,
};
use askama::Template;

use crate::types::*;
use crate::templates::*;

pub async fn home() -> Html<String> {
    let r = IndexTemplate {};

    let template = match r.render() {
        Ok(rendered_template) => rendered_template,
        Err(err) => {
            let err_msg = format!("Failed to render index.html with following error: {}", err);
            tracing::error!(err_msg);
            StatusCode::BAD_REQUEST.to_string()
        }
    };

    tracing::debug!("Serving index.html");
    Html(template)
}

pub async fn profile(Query(profile): Query<Profile>) -> Html<String> {
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
            StatusCode::BAD_REQUEST.to_string()
        }
    };

    tracing::debug!("Serving profile");
    Html(template)
}
