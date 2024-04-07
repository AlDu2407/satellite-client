use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestData {
    pub url: String,
    pub body: Option<serde_json::Value>,
    pub secure: bool,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    Get(RequestData),
    Post(RequestData),
}

impl RequestData {
    pub fn prefixed_url(&self) -> String {
        if self.url.starts_with("http://") || self.url.starts_with("https://") {
            return self.url.clone();
        }

        let prefix = if self.secure { "https://" } else { "http://" };
        format!("{prefix}{}", self.url)
    }
}
