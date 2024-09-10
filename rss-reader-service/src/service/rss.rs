use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};

#[derive(Deserialize, Serialize, Debug)]
pub struct RSSItem {
  pub link: String,
  #[serde(rename = "pubDate")]
  pub pub_date: String,
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
                                
pub fn rss_to_json(value: Value) -> RSSObject {
  from_value(value).unwrap()
}