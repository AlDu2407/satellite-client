use serde::Deserialize;

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
#[ts(export_to = "base-req.ts")]
pub struct BaseReq {
    pub url: String,
    pub secure: bool,
}

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
#[ts(export_to = "get-req.ts")]
pub struct GetReq {
    #[serde(flatten)]
    pub base: BaseReq,
}

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
#[ts(export_to = "post-req.ts")]
pub struct PostReq {
    #[serde(flatten)]
    pub base: BaseReq,
    pub body: serde_json::Value,
}

#[derive(Debug, Deserialize, ts_rs::TS)]
#[ts(export)]
#[ts(export_to = "request.ts")]
#[serde(tag = "method")]
#[serde(rename_all = "UPPERCASE")]
pub enum Request {
    Get(GetReq),
    Post(PostReq),
}

impl BaseReq {
    pub fn prefixed_url(&self) -> String {
        if self.url.starts_with("http://") || self.url.starts_with("https://") {
            return self.url.clone();
        }

        let prefix = if self.secure { "https://" } else { "http://" };
        format!("{prefix}{}", self.url)
    }
}
