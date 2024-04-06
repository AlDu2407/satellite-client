// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
async fn execute_request(url: &str) -> Result<serde_json::Value, ()> {
    let resp = reqwest::get(url)
        .await
        .expect("must have response")
        .json::<serde_json::Value>()
        .await
        .expect("must be map");

    Ok(resp)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
