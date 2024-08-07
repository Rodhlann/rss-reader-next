use axum::{response::IntoResponse, routing::get, Json, Router};
use reqwest::StatusCode;
use serde_json::{json, Value};

use crate::{db::{self, get_feeds, persist_feed}, feeds::fetch_feed_json};

async fn feeds() -> impl IntoResponse {
    let feeds: Vec<db::Feed> = get_feeds().expect("TODO");
    let mut values: Vec<Value> = vec![];
    for feed in feeds {
        values.push(fetch_feed_json(&feed.url).await);
    }
    Json(json!(values))
}

async fn add_feed(Json(input): Json<db::Feed>) -> impl IntoResponse {
    let _ = persist_feed(input);
    StatusCode::CREATED
}

pub fn app() -> Router {
    Router::new()
        .route("/", get(feeds).post(add_feed))
}
