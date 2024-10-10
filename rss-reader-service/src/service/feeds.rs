use std::fmt::Display;

use axum::{extract::{Path, Query, State}, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{db::{FeedDataSource, FeedInput}, service::AtomEntry, AppState};

use super::{atom_to_json, fetch_feed_json, rss_to_json};

#[derive(Deserialize, Serialize, Debug)]
pub enum FeedError {
  Message(String),
}

impl Display for FeedError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FeedError::Message(msg) => write!(f, "Feed parse error: {}", msg),
    }    
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Entry {
  pub title: String,
  pub url: String,
  pub created_date: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Feed {
  pub name: String,
  pub category: String,
  pub entries: Vec<Entry>
}

impl Feed {
  pub fn try_from_rss(name: String, category: String, duration: Duration, max_entries: usize, value: Value) -> Result<Self, FeedError> {
    let items = rss_to_json(value)
      .map_err(|e| FeedError::Message(e.to_string()))?
      .rss.channel.item;

    let items_in_duration: Vec<_> = items.iter().filter(|item| duration.compare(item.pub_date)).collect();
    let item_count = max_entries.min(items_in_duration.len());
    let trimmed_items = &items_in_duration[..item_count];

    Ok(Self {
      name,
      category,
      entries: trimmed_items.iter().map(|item| Entry {
        title: item.title.to_string(),
        url: item.link.to_string(),
        created_date: item.pub_date.to_string()
      }).collect()
    })
  }

  pub fn try_from_atom(name: String, category: String, duration: Duration, max_entries: usize, value: Value) -> Result<Self, FeedError> {
    let items = atom_to_json(value)
      .map_err(|e| FeedError::Message(e.to_string()))?
      .feed.entry;

    fn date(entry: &AtomEntry) -> DateTime<Utc> {
      entry.published
          .or(entry.updated)
          .unwrap_or(DateTime::UNIX_EPOCH)
    }

    let items_in_duration: Vec<_> = items.iter().filter(|item| duration.compare(date(item))).collect();
    let item_count = max_entries.min(items_in_duration.len());
    let trimmed_items = &items_in_duration[..item_count];

    Ok(Self {
      name,
      category,
      entries: trimmed_items.iter().map(|item| Entry {
        title: item.title.to_string(),
        url: item.link.to_string(),
        created_date: date(item).to_string()
      }).collect()
    })
  }
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Duration {
  DAY,
  WEEK,
  MONTH,
  YEAR
}

impl Duration {
  pub fn compare(&self, date: chrono::DateTime<Utc>) -> bool {
    let now = Utc::now();
    match self {
      Duration::DAY => date >= now - chrono::Duration::days(1),
      Duration::WEEK => date >= now - chrono::Duration::weeks(1),
      Duration::MONTH => date >= now - chrono::Duration::weeks(4),
      Duration::YEAR => date >= now - chrono::Duration::weeks(52),
    }
  } 
}

#[derive(Deserialize, Debug)]
pub struct FeedsParam {
  pub duration: Option<Duration>,
  pub max_entries: Option<usize>
}

pub async fn get_rss_feeds(
  State(state): State<AppState>,
  Query(params): Query<FeedsParam>
) -> Result<impl IntoResponse, impl IntoResponse> {
  println!("Fetching all RSS feed data");

  let feed_db = FeedDataSource::new(state.db.clone());
  let duration = params.duration.unwrap_or(Duration::WEEK);
  let max_entries = params.max_entries.unwrap_or(5);

  match feed_db.get_feeds().await {
    Ok(feeds) => {
      let fetch_futures = feeds.into_iter().map(|feed| {
        println!("Preparing feed: {}", feed.name);

        let db = state.db.clone();
        async move {
          let result = fetch_feed_json(
            &feed.name, 
            &feed.category, 
            &feed.url,
            duration,
            max_entries,
            db
          ).await;
          (feed.name, result)
        }
      }).collect::<Vec<_>>();

      let results = join_all(fetch_futures).await;

      let mut values: Vec<Feed> = Vec::new();
      results.into_iter().for_each(|(name, result)| {
        match result {
          Ok(feed) => values.push(feed),
          Err(err) => {
            println!("Failed to fetch feed: {} - {:?}", name, err)
          }
        }
      });
      
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

pub async fn batch_create_feeds(
  State(state): State<AppState>,
  Json(feeds): Json<Vec<FeedInput>>
) -> Result<impl IntoResponse, impl IntoResponse> {
  let feed_db = FeedDataSource::new(state.db);
  match feed_db.batch_create_feeds(feeds).await {
    Ok(feeds) => Ok(Json(feeds)),
    Err(e) => Err(e.into_response())
  }
}

pub async fn create_feed(
  State(state): State<AppState>,
  Json(feed): Json<FeedInput>
) -> Result<impl IntoResponse, impl IntoResponse> {
  let feed_db = FeedDataSource::new(state.db);
  match feed_db.create_feed(feed).await {
    Ok(feeds) => Ok(Json(feeds)),
    Err(e) => Err(e.into_response())
  }
}

pub async fn delete_feed(
  State(state): State<AppState>,
  Path(id): Path<i32>
) -> Result< impl IntoResponse, impl IntoResponse> {
  let feed_db = FeedDataSource::new(state.db);
  feed_db.delete_feed(id).await
}