use std::io;

use axum::response::{Response, IntoResponse};
use quickxml_to_serde::{xml_string_to_json, Config};
use reqwest::StatusCode;
use rss::Channel;
use serde_json::Value;

// TODO: deserialize RSS and Atom feeds, return only relevant JSON content to client
// TODO: Allow for filtering by query param? 
pub struct JsonFeed {

}

#[derive(Debug)]
#[allow(dead_code)]
pub enum FetchXmlError {
  Network(reqwest::Error),
  Parse(rss::Error),
  Io(io::Error)
}

impl From<reqwest::Error> for FetchXmlError {
  fn from(error: reqwest::Error) -> Self {
      FetchXmlError::Network(error)
  }
}

impl From<rss::Error> for FetchXmlError {
  fn from(error: rss::Error) -> Self {
      FetchXmlError::Parse(error)
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

async fn fetch_feed_xml(route: &String) -> Result<Channel, FetchXmlError> {
  let content = reqwest::get(route).await?.bytes().await?;
  let channel = Channel::read_from(&content[..])?;
  Ok(channel)
}

pub async fn fetch_feed_json(feed_url: &String) -> Value {
  let xml_channel = fetch_feed_xml(feed_url).await.unwrap();
  xml_string_to_json(xml_channel.to_string(), &Config::new_with_defaults()).unwrap()
}