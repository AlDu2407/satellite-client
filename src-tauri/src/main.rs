// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

#[tauri::command]
async fn execute_request(url: &str) -> Result<String> {
    let resp = reqwest::get(url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    let result = serde_json::to_string(&resp)?;
    Ok(result)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
