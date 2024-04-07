use serde::Serialize;

use super::request::{Request, RequestData};

#[derive(Serialize)]
pub enum SatelliteError {
    ClientError,
}

impl From<reqwest::Error> for SatelliteError {
    fn from(_: reqwest::Error) -> Self {
        SatelliteError::ClientError
    }
}

#[tauri::command]
pub async fn execute_request(request: Request) -> Result<serde_json::Value, SatelliteError> {
    match request {
        Request::Get(get_data) => execute_get(&get_data).await,
        Request::Post(_) => todo!(),
    }
}

async fn execute_post(request: &RequestData) -> Result<serde_json::Value, SatelliteError> {
    let client = reqwest::Client::new();
    let response = client
        .post(&request.prefixed_url())
        .json(&request.body)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(response)
}

async fn execute_get(request: &RequestData) -> Result<serde_json::Value, SatelliteError> {
    let resp = reqwest::get(&request.prefixed_url())
        .await?
        .json::<serde_json::Value>()
        .await?;

    Ok(resp)
}
