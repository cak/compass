use crate::models::Response;
use crate::core::CompassError;
use reqwest;

pub fn get(url: &str) -> Result<Response, CompassError> {
    let mut res = reqwest::get(url)?;
    let body = res.text()?;

    let response = Response {
        url: url.to_owned(),
        body,
    };

    Ok(response)
}
