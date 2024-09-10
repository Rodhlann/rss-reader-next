use axum::{extract::{Path, State}, response::IntoResponse, Json};
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
  pub entries: Vec<Entry>
}

impl Feed {
  pub fn from_rss(title: String, value: Value) -> Self {
    let obj = rss_to_json(value);
    Self {
      title,
      entries: obj.rss.channel.item.iter().map(|item| Entry {
        title: item.title.to_string(),
        url: item.link.to_string(),
        // TODO: Unify date format
        created_date: item.pub_date.to_string()
      }).collect()
    }
  }

  pub fn from_atom(title: String, value: Value) -> Self {
    let obj = atom_to_json(value);
    Self {
      title,
      entries: obj.feed.entry.iter().map(|item| Entry {
        title: item.title.to_string(),
        url: item.link.iter().filter(|link| link.link_type == "text/html").nth(0).unwrap().href.to_string(),
        // TODO: Unify date format
        created_date: item.updated.to_string()
      }).collect()
    }
  }
}

pub async fn get_rss_feeds(
  State(state): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
  println!("Fetching all RSS feed data");

  let feed_db = FeedDataSource::new(state.db);
  let mut values: Vec<Feed> = Vec::new();

  match feed_db.get_feeds().await {
    Ok(feeds) => {
      for feed in feeds {
        println!("Preparing feed: {}", feed.name);
        // TODO: parallelize
        values.push(
          fetch_feed_json(&feed.name, &feed.url)
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