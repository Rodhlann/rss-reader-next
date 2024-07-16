use quickxml_to_serde::{xml_string_to_json, Config};
use serde_json::Value;

pub fn xml_to_json(xml_contents: String) -> Value {
    xml_string_to_json(xml_contents, &Config::new_with_defaults()).unwrap()
}
