use core::fmt;
use std::io;

use axum::response::{Response, IntoResponse};
use chrono::{Duration, Utc};
use quickxml_to_serde::{xml_string_to_json, Config};
use reqwest::StatusCode;
use sqlx::PgPool;

use crate::db::{CacheDataSource, CacheInput};

use super::Feed;

#[derive(Debug)]
#[allow(dead_code)]
pub enum FetchXmlError {
  Network(reqwest::Error),
  Parse(String),
  Io(io::Error),
  Cache(String)
}

impl From<reqwest::Error> for FetchXmlError {
  fn from(error: reqwest::Error) -> Self {
      FetchXmlError::Network(error)
  }
}

impl From<io::Error> for FetchXmlError {
  fn from(error: io::Error) -> Self {
      FetchXmlError::Io(error)
  }
}

impl IntoResponse for FetchXmlError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
          FetchXmlError::Network(_) => (StatusCode::BAD_GATEWAY, "Failed to fetch feed XML."),
          FetchXmlError::Parse(_) => (StatusCode::BAD_REQUEST, "Failed to parse feed XML."),
          FetchXmlError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error."),
          FetchXmlError::Cache(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Cache error.")
        };
        (status, error_message).into_response()
    }
}

async fn fetch_feed_xml(route: String) -> Result<String, FetchXmlError> {
  let response = reqwest::get(route).await.map_err(FetchXmlError::from)?;
  let content = response.text().await.map_err(FetchXmlError::from)?;
  Ok(content)
}

pub async fn fetch_feed_json(
  feed_name: &str, 
  feed_category: &str, 
  feed_url: &str,
  max_entries: usize,
  db: &PgPool,
) -> Result<Feed, FetchXmlError> {
  let cache = CacheDataSource::new(db.to_owned());

  let cached = cache.get_cached_value(feed_name.to_string()).await
    .map_err(|e| FetchXmlError::Cache(e.to_string()))?;

  let xml_string = match cached {
    Some(cached_value) => {
      if cached_value.created_date + Duration::minutes(10) > Utc::now() {
        cached_value.xml_string
      } else {
        let result = fetch_feed_xml(feed_url.to_string()).await?;
        cache.clear_cache(feed_name.to_string()).await
          .map_err(|e| FetchXmlError::Cache(e.to_string()))?;
        cache.cache_value(CacheInput { name: feed_name.to_string(), xml_string: result.clone() }).await
          .map_err(|e| FetchXmlError::Cache(e.to_string()))?;
        result
      }
    } 
    None => {
      let result = fetch_feed_xml(feed_url.to_string()).await?;
      cache.cache_value(CacheInput { name: feed_name.to_string(), xml_string: result.clone() }).await
        .map_err(|e| FetchXmlError::Cache(e.to_string()))?;
      result
    }
  };

  let value = xml_string_to_json(xml_string.clone(), &Config::new_with_defaults())
    .map_err(|e| FetchXmlError::Parse(e.to_string()))?;

  // TODO: cache value
  if xml_string.contains("<rss") {
    Ok(Feed::from_rss(feed_name.to_string(), feed_category.to_string(), max_entries, value))
  } else if xml_string.contains("<feed") {
    Ok(Feed::from_atom(feed_name.to_string(), feed_category.to_string(), max_entries, value))
  } else {
    Err(FetchXmlError::Parse("Unknown feed syntax".to_string()))
  }
}