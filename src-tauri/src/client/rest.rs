use serde::Serialize;

use super::{
    request::{DeleteReq, GetReq, PostReq, PutReq, Request},
    response::SatelliteResponse,
};

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
pub async fn execute_request(request: Request) -> Result<SatelliteResponse, SatelliteError> {
    match request {
        Request::Get(get_req) => execute_get(&get_req).await,
        Request::Post(post_req) => execute_post(&post_req).await,
        Request::Put(put_req) => execute_put(&put_req).await,
        Request::Delete(delete_req) => execute_delete(&delete_req).await,
    }
}

async fn execute_post(request: &PostReq) -> Result<SatelliteResponse, SatelliteError> {
    let client = reqwest::Client::new();
    let response = client
        .post(&request.base.prefixed_url())
        .json(&request.body)
        .send()
        .await?;

    Ok(SatelliteResponse::from_response(response).await?)
}

async fn execute_get(request: &GetReq) -> Result<SatelliteResponse, SatelliteError> {
    let client = reqwest::Client::new();
    let response = client.get(&request.base.prefixed_url()).send().await?;

    Ok(SatelliteResponse::from_response(response).await?)
}

async fn execute_put(request: &PutReq) -> Result<SatelliteResponse, SatelliteError> {
    let client = reqwest::Client::new();
    let response = client
        .put(&request.base.prefixed_url())
        .json(&request.body)
        .send()
        .await?;

    Ok(SatelliteResponse::from_response(response).await?)
}

async fn execute_delete(request: &DeleteReq) -> Result<SatelliteResponse, SatelliteError> {
    let client = reqwest::Client::new();
    let response = client.delete(&request.base.prefixed_url()).send().await?;

    Ok(SatelliteResponse::from_response(response).await?)
}
