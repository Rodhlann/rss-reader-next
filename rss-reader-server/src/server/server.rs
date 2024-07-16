use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

use crate::{db::get_feeds, feeds::fetch_feed_json};

async fn root() -> Json<Value> {
    let feeds = get_feeds();
    let mut values: Vec<Value> = vec![];
    for feed in feeds {
        values.push(fetch_feed_json(&feed.url).await);
    }
    Json(json!(values))
}

pub fn app() -> Router {
    Router::new().route("/", get(root))
}
