use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Compass {
    pub response: Response,
    pub details: Details,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub url: String,
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct Details {
    pub title: Option<String>,
    pub links: Vec<String>,
    pub scripts: Vec<String>,
}
