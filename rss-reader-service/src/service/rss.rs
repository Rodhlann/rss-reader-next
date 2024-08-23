use std::io;

use axum::response::{Response, IntoResponse};
use quickxml_to_serde::{xml_string_to_json, Config};
use reqwest::StatusCode;
use rss::Channel;
use serde_json::Value;

#[derive(Debug)]
#[allow(dead_code)]
pub enum FetchRssError {
  Network(reqwest::Error),
  Parse(rss::Error),
  Io(io::Error)
}

impl From<reqwest::Error> for FetchRssError {
  fn from(error: reqwest::Error) -> Self {
      FetchRssError::Network(error)
  }
}

impl From<rss::Error> for FetchRssError {
  fn from(error: rss::Error) -> Self {
      FetchRssError::Parse(error)
  }
}

impl From<io::Error> for FetchRssError {
  fn from(error: io::Error) -> Self {
      FetchRssError::Io(error)
  }
}

impl IntoResponse for FetchRssError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
          FetchRssError::Network(_) => (StatusCode::BAD_GATEWAY, "Failed to fetch RSS feed."),
          FetchRssError::Parse(_) => (StatusCode::BAD_REQUEST, "Failed to parse RSS feed."),
          FetchRssError::Io(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error."),
        };
        (status, error_message).into_response()
    }
}

async fn fetch_rss_xml(route: &String) -> Result<Channel, FetchRssError> {
  let content = reqwest::get(route).await?.bytes().await?;
  let channel = Channel::read_from(&content[..])?;
  Ok(channel)
}

pub async fn fetch_rss_feed_json(feed_url: &String) -> Value {
  let xml_channel = fetch_rss_xml(feed_url).await.unwrap();
  xml_string_to_json(xml_channel.to_string(), &Config::new_with_defaults()).unwrap()
}