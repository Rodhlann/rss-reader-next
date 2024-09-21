use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::{from_value, Value};

#[derive(Deserialize, Serialize, Debug)]
pub enum AtomError {
  Message(String),
}

impl Display for AtomError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AtomError::Message(msg) => write!(f, "Atom: {}", msg),
    }    
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AtomLink {
  #[serde(rename = "@href")]
  pub href: String,
  #[serde(rename = "@type")]
  pub link_type: String, 
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AtomEntry {
  #[serde(deserialize_with = "link")]
  pub link: Vec<AtomLink>,
  #[serde(deserialize_with = "updated_date_time")]
  pub updated: DateTime<Utc>,
  pub title: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AtomRoot {
  pub entry: Vec<AtomEntry>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AtomFeed {
  pub feed: AtomRoot
}

fn link<'de, D>(deserializer: D) -> Result<Vec<AtomLink>, D::Error>
where
  D: Deserializer<'de>,
{
  #[derive(Deserialize)]
  #[serde(untagged)]
  enum LinkMultiType {
      Vec(Vec<AtomLink>),
      Single(AtomLink),
  }

  match LinkMultiType::deserialize(deserializer)? {
    LinkMultiType::Vec(v) => Ok(v),
    LinkMultiType::Single(link) => Ok(vec![link]),
  }
}

fn updated_date_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  // Atom Date: 2024-07-23T07:28:00+00:00
  if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
    return Ok(dt.with_timezone(&Utc));
  }

  Err(de::Error::custom(&format!("Failed to parse Atom date: {}", &s)))
}

pub fn atom_to_json(value: Value) -> Result<AtomFeed, AtomError> {
  from_value(value).map_err(|e| AtomError::Message(e.to_string()))
}