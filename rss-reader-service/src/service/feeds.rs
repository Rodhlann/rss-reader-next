use axum::{extract::{Path, State}, response::IntoResponse, Json};
use serde_json::{json, Value};

use crate::{db::{FeedDataSource, FeedInput}, AppState};

use super::fetch_feed_json;

pub async fn get_rss_feeds(
  State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
  println!("Fetching all RSS feed data");

  let feed_db = FeedDataSource::new(state.db);
  let mut values: Vec<Value> = Vec::new();

  match feed_db.get_feeds().await {
    Ok(feeds) => {
      for feed in feeds {
        println!("Preparing feed: {}", feed.name);
        // TODO: parallelize
        values.push(fetch_feed_json(&feed.url).await)
      }
      Ok(Json(json!(values)))
    },
    Err(e) => Err(e.into_response())
  }
}

pub async fn get_raw_feeds(
  State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
  println!("Fetching raw RSS feed data");

  let feed_db = FeedDataSource::new(state.db);
  match feed_db.get_feeds().await {
    Ok(feeds) => Ok(Json(feeds)),
    Err(e) => Err(e.into_response())
  }
}

pub async fn create_feed(
  State(state): State<AppState>,
  Json(feed): Json<FeedInput>
) -> Result<impl IntoResponse, impl IntoResponse> {
  let feed_db = FeedDataSource::new(state.db);
  feed_db.create_feed(feed).await
}

pub async fn delete_feed(
  State(state): State<AppState>,
  Path(id): Path<i32>
) -> Result< impl IntoResponse, impl IntoResponse> {
  let feed_db = FeedDataSource::new(state.db);
  feed_db.delete_feed(id).await
}