use rss::Channel;
use serde_json::Value;
use std::error::Error;

use super::xml_to_json;

async fn fetch_xml(route: &String) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(route).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub async fn fetch_feed_json(feed_url: &String) -> Value {
    let xml_channel = fetch_xml(feed_url).await.unwrap();
    xml_to_json(xml_channel.to_string())
}
