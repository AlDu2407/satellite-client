use reqwest::Method;

use super::{
    request::{
        ConnectReq, DeleteReq, GetReq, HeadReq, OptionsReq, PatchReq, PostReq, PutReq, Request,
        TraceReq,
    },
    response::{Result, SatelliteResponse},
};

#[tauri::command]
pub async fn execute_request(request: Request) -> Result<SatelliteResponse> {
    match request {
        Request::Get(get_req) => execute_get(&get_req).await,
        Request::Post(post_req) => execute_post(&post_req).await,
        Request::Put(put_req) => execute_put(&put_req).await,
        Request::Delete(delete_req) => execute_delete(&delete_req).await,
        Request::Patch(patch_req) => execute_patch(&patch_req).await,
        Request::Head(head_req) => execute_head(&head_req).await,
        Request::Connect(connect_req) => execute_connect(&connect_req).await,
        Request::Options(options_req) => execute_options(&options_req).await,
        Request::Trace(trace_req) => execute_trace(&trace_req).await,
    }
}

fn create_client() -> reqwest::Client {
    reqwest::Client::new()
}

async fn execute_post(request: &PostReq) -> Result<SatelliteResponse> {
    let client = create_client();
    let response = client
        .post(&request.base.prefixed_url())
        .json(&request.body)
        .send()
        .await?;

    SatelliteResponse::from_response(response).await
}

async fn execute_get(request: &GetReq) -> Result<SatelliteResponse> {
    let client = create_client();
    let response = client.get(&request.base.prefixed_url()).send().await?;

    SatelliteResponse::from_response(response).await
}

async fn execute_put(request: &PutReq) -> Result<SatelliteResponse> {
    let client = create_client();
    let response = client
        .put(&request.base.prefixed_url())
        .json(&request.body)
        .send()
        .await?;

    SatelliteResponse::from_response(response).await
}

async fn execute_delete(request: &DeleteReq) -> Result<SatelliteResponse> {
    let client = create_client();
    let response = client.delete(&request.base.prefixed_url()).send().await?;

    SatelliteResponse::from_response(response).await
}

async fn execute_patch(request: &PatchReq) -> Result<SatelliteResponse> {
    let client = create_client();
    let response = client
        .patch(&request.base.prefixed_url())
        .json(&request.body)
        .send()
        .await?;

    SatelliteResponse::from_response(response).await
}

async fn execute_head(request: &HeadReq) -> Result<SatelliteResponse> {
    let client = create_client();
    let response = client.head(&request.base.prefixed_url()).send().await?;

    SatelliteResponse::from_response(response).await
}

async fn execute_connect(request: &ConnectReq) -> Result<SatelliteResponse> {
    let client = create_client();
    let response = client
        .request(Method::CONNECT, &request.base.prefixed_url())
        .send()
        .await?;

    SatelliteResponse::from_response(response).await
}

async fn execute_options(request: &OptionsReq) -> Result<SatelliteResponse> {
    let client = create_client();
    let response = client
        .request(Method::OPTIONS, &request.base.prefixed_url())
        .send()
        .await?;

    SatelliteResponse::from_response(response).await
}

async fn execute_trace(request: &TraceReq) -> Result<SatelliteResponse> {
    let response = create_client()
        .request(Method::TRACE, &request.base.prefixed_url())
        .send()
        .await?;

    SatelliteResponse::from_response(response).await
}
