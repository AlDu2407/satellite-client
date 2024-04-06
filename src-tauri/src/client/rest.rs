#[tauri::command]
pub async fn execute_request(url: &str) -> Result<serde_json::Value, ()> {
    let resp = reqwest::get(url)
        .await
        .expect("must have response")
        .json::<serde_json::Value>()
        .await
        .expect("must be map");

    Ok(resp)
}
