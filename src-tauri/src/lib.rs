#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::commands::{
    close_workspace, 
    create_profile, 
    create_brief_case,
    load_profiles, 
    load_briefcases,
    save_profiles, 
    save_briefcases,
    save_all_data,
    open_workspace, 
    test_command
};

use crate::state::AppState;
use tauri::{Manager, Runtime};

mod commands;
mod models;
mod project_errors;
mod screenshot;
mod state;
mod storage;
mod workspace;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let state = tauri::async_runtime::block_on(async {
                AppState::new(app).await.expect("Failed to create state")
            });
            app.manage(state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            test_command,
            close_workspace,
            create_profile,
            create_brief_case,
            load_profiles,
            load_briefcases,
            save_profiles,
            save_briefcases,
            save_all_data,
            open_workspace,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
