// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;

use client::rest::execute_request;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
