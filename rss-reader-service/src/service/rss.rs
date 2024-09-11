use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{from_value, Value};

#[derive(Deserialize, Serialize, Debug)]
pub struct RSSItem {
  pub link: String,
  #[serde(rename = "pubDate", deserialize_with = "updated_date_time")]
  pub pub_date: DateTime<Utc>,
  pub title: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RSSChannel {
  pub item: Vec<RSSItem>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RSSRoot {
  pub channel: RSSChannel
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RSSObject {
  pub rss: RSSRoot
}

fn updated_date_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  // Atom Date: Tue, 03 Sep 2024 13:51:48 GMT
  let dt = NaiveDateTime::parse_from_str(&s, "%a, %d %b %Y %H:%M:%S GMT").map_err(serde::de::Error::custom)?;
  Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
}
               
pub fn rss_to_json(value: Value) -> RSSObject {
  from_value(value).unwrap()
}