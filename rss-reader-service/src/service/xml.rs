use std::io;

use axum::response::{Response, IntoResponse};
use quickxml_to_serde::{xml_string_to_json, Config};
use reqwest::StatusCode;
use sqlx::PgPool;

use crate::db::{CacheDataSource, CacheInput};

use super::{fetch_cached, Feed};

#[derive(Debug)]
#[allow(dead_code)]
pub enum FetchXmlError {
  Network(reqwest::Error),
  Io(io::Error),
  Parse(String),
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
          FetchXmlError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error."),
          FetchXmlError::Parse(_) => (StatusCode::BAD_REQUEST, "Failed to parse feed XML."),
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
  db: PgPool,
) -> Result<Feed, FetchXmlError> {
  let xml_string: String = if let Some(cache_value) = fetch_cached(feed_name, &db).await
    .map_err(|e| FetchXmlError::Cache(e.to_string()))? 
  {
    // If cached xml_string exists return cached value
    println!("Resolving cached feed: {feed_name}");
    cache_value.xml_string
  } else {
    // Else fetch xml_string, cache it, and return new value
    println!("No cached feed, fetching live: {feed_name}");
    let new_xml_string = fetch_feed_xml(feed_url.to_string()).await?;
    let cache = CacheDataSource::new(&db.to_owned());
    cache.cache_value(CacheInput { name: feed_name.to_string(), xml_string: new_xml_string.to_string() }).await
      .map_err(|e| FetchXmlError::Cache(e.to_string()))?;
    new_xml_string
  };

  let value = xml_string_to_json(xml_string.clone(), &Config::new_with_defaults())
    .map_err(|e| FetchXmlError::Parse(e.to_string()))?;

  if xml_string.contains("<rss") {
    Ok(Feed::from_rss(feed_name.to_string(), feed_category.to_string(), max_entries, value))
  } else if xml_string.contains("<feed") {
    Ok(Feed::from_atom(feed_name.to_string(), feed_category.to_string(), max_entries, value))
  } else {
    Err(FetchXmlError::Parse("Unknown feed syntax".to_string()))
  }
}