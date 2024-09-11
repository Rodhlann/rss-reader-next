use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{from_value, Value};

#[derive(Deserialize, Serialize, Debug)]
pub struct AtomLink {
  #[serde(rename = "@href")]
  pub href: String,
  #[serde(rename = "@type")]
  pub link_type: String, 
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AtomEntry {
  pub link: Vec<AtomLink>,
  #[serde(deserialize_with = "updated_date_time")]
  pub updated: DateTime<Utc>,
  // pub updated: String,
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

fn updated_date_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  // Atom Date: 2024-07-23T07:28:00+00:00
  let dt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%:z").map_err(serde::de::Error::custom)?;
  Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
}

pub fn atom_to_json(value: Value) -> AtomFeed {
  from_value(value).unwrap()
}