use serde::Serialize;

use super::request::{GetReq, PostReq, Request};

#[derive(Serialize, ts_rs::TS)]
#[ts(export)]
#[ts(export_to = "error.ts")]
#[serde(tag = "type", content = "error")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SatelliteError {
    ClientError(String),
}

impl From<reqwest::Error> for SatelliteError {
    fn from(err: reqwest::Error) -> Self {
        SatelliteError::ClientError(err.to_string())
    }
}

#[tauri::command]
pub async fn execute_request(request: Request) -> Result<serde_json::Value, SatelliteError> {
    match request {
        Request::Get(get_data) => execute_get(&get_data).await,
        Request::Post(post_data) => execute_post(&post_data).await,
    }
}

async fn execute_post(request: &PostReq) -> Result<serde_json::Value, SatelliteError> {
    let client = reqwest::Client::new();
    let response = client
        .post(&request.base.prefixed_url())
        .json(&request.body)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response)
}

async fn execute_get(request: &GetReq) -> Result<serde_json::Value, SatelliteError> {
    let resp = reqwest::get(&request.base.prefixed_url())
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(resp)
}
