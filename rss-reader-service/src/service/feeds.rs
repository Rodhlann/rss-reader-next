use axum::{extract::{Path, Query, State}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{db::{FeedDataSource, FeedInput}, AppState};

use super::{atom_to_json, fetch_feed_json, rss_to_json};

#[derive(Deserialize, Serialize, Debug)]
pub struct Entry {
  pub title: String,
  pub url: String,
  pub created_date: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Feed {
  pub title: String,
  pub category: String,
  pub entries: Vec<Entry>
}

impl Feed {
  pub fn from_rss(title: String, category: String, max_entries: usize, value: Value) -> Self {
    let items = rss_to_json(value).rss.channel.item;
    let item_count = max_entries.min(items.len());
    let trimmed_items = &items[..item_count];

    Self {
      title,
      category,
      entries: trimmed_items.iter().map(|item| Entry {
        title: item.title.to_string(),
        url: item.link.to_string(),
        created_date: item.pub_date.to_string()
      }).collect()
    }
  }

  pub fn from_atom(title: String, category: String, max_entries: usize, value: Value) -> Self {
    let items = atom_to_json(value).feed.entry;
    let item_count = max_entries.min(items.len());
    let trimmed_items = &items[..item_count];
    Self {
      title,
      category,
      entries: trimmed_items.iter().map(|item| Entry {
        title: item.title.to_string(),
        url: item.link.iter().filter(|link| link.link_type == "text/html").nth(0).unwrap().href.to_string(),
        created_date: item.updated.to_string()
      }).collect()
    }
  }
}

#[derive(Deserialize, Debug)]
pub struct FeedsParam {
  pub max_entries: Option<usize>
}

pub async fn get_rss_feeds(
  State(state): State<AppState>,
  Query(params): Query<FeedsParam>
) -> Result<impl IntoResponse, impl IntoResponse> {
  println!("Fetching all RSS feed data");

  let feed_db = FeedDataSource::new(state.db);
  let max_entries = params.max_entries.unwrap_or(5);

  let mut values: Vec<Feed> = Vec::new();

  match feed_db.get_feeds().await {
    Ok(feeds) => {
      for feed in feeds {
        println!("Preparing feed: {}", feed.name);
        // TODO: parallelize
        values.push(
          fetch_feed_json(&feed.name, &feed.category, &feed.url, max_entries)
            .await
            .map_err(|e| e.into_response())?
        )
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