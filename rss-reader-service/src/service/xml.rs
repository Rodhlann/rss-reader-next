use std::io;

use axum::response::{Response, IntoResponse};
use quickxml_to_serde::{xml_string_to_json, Config};
use reqwest::StatusCode;

use super::{Feed};

#[derive(Debug)]
#[allow(dead_code)]
pub enum FetchXmlError {
  Network(reqwest::Error),
  Parse(String),
  Io(io::Error)
}

impl From<reqwest::Error> for FetchXmlError {
  fn from(error: reqwest::Error) -> Self {
      FetchXmlError::Network(error)
  }
}

impl From<rss::Error> for FetchXmlError {
  fn from(error: rss::Error) -> Self {
      FetchXmlError::Parse(error.to_string())
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
        };
        (status, error_message).into_response()
    }
}

async fn fetch_feed_xml(route: String) -> Result<String, FetchXmlError> {
  let content = reqwest::get(route).await?.text().await?;
  Ok(content)
}

pub async fn fetch_feed_json(
  feed_name: &String, 
  feed_category: &String, 
  feed_url: &String,
  max_entries: usize
) -> Result<Feed, FetchXmlError> {
  let xml_string = fetch_feed_xml(feed_url.to_string()).await?;
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