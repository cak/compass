use crate::http;
use crate::models::Compass;
use crate::scrape;
use reqwest;
use serde_json::{json, to_string_pretty, value::Value};

#[derive(Debug)]
pub struct CompassError {
    pub kind: String,
    pub message: String,
}

impl From<reqwest::Error> for CompassError {
    fn from(error: reqwest::Error) -> Self {
        CompassError {
            kind: String::from("reqwest"),
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for CompassError {
    fn from(error: serde_json::Error) -> Self {
        CompassError {
            kind: String::from("serde_json"),
            message: error.to_string(),
        }
    }
}

pub fn fetch(url: &str) -> Result<Compass, CompassError> {
    let response = http::get(&url)?;
    let details = scrape::scraper(&response.body);
    let compass = Compass { response, details };
    Ok(compass)
}

pub fn fetch_json(url: &str) -> Result<Value, CompassError> {
    let compass = fetch(&url)?;
    let compass_json = json!(compass);
    Ok(compass_json)
}

pub fn fetch_pretty(url: &str) -> Result<String, CompassError> {
    let compass_json = fetch_json(&url)?;
    let compass_pretty = to_string_pretty(&compass_json)?;
    Ok(compass_pretty)
}
