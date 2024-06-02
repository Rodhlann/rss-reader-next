mod db;
mod feeds;

use axum::{routing::get, Json, Router};
use db::get_feeds;
use feeds::fetch_feed_json;
use rusqlite::Connection;
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let conn = Connection::open("feeds.db").expect("DB init failed");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS rss_feeds (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      title TEXT NOT NULL UNIQUE,
      url TEXT NOT NULL UNIQUE,
      category TEXT CHECK(category IN ('code', 'tech', 'ocean')) NOT NULL
    )",
        (),
    )
    .expect("DB failed to execute");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new().route("/", get(root))
}

async fn root() -> Json<Value> {
    let feeds = get_feeds();
    let mut values: Vec<Value> = vec![];
    for feed in feeds {
        values.push(fetch_feed_json(&feed.url).await);
    }
    Json(json!(values))
}
