#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::state::State;
use crate::storage::Storage;
use tauri::Manager;

mod models;

mod project_errors;
mod state;
mod storage;
mod tests;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let state = tauri::async_runtime::block_on(async {
                let storage = Storage::new().await.expect("Failed to init storage");
                State::new(storage).await
            });

            app.manage(state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
