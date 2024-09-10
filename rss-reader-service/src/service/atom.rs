use serde::{Deserialize, Serialize};
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
  pub updated: String,
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

pub fn atom_to_json(value: Value) -> AtomFeed {
  from_value(value).unwrap()
}