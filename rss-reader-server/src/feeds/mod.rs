use std::error::Error;

use quickxml_to_serde::{xml_string_to_json, Config};
use rss::Channel;
use serde_json::Value;

async fn fetch_xml(route: &String) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(route).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

fn xml_to_json(xml_contents: String) -> Value {
    xml_string_to_json(xml_contents, &Config::new_with_defaults()).unwrap()
}

pub async fn fetch_feed_json(feed_url: &String) -> Value {
    let xml_channel = fetch_xml(feed_url).await.unwrap();
    xml_to_json(xml_channel.to_string())
}
