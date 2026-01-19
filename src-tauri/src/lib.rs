#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::commands::{
    change_webview_url, close_workspace, create_brief_case, create_profile, execute_next_task,
    get_panel_data, launch_panel_window, load_briefcases, load_profiles, next_task_execution,
    prev_task_execution, save_all_data, save_briefcases, save_profiles, start_automation,
    test_command, TaskExecutionResult,
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
            launch_panel_window,
            get_panel_data,
            start_automation,
            next_task_execution,
            prev_task_execution,
            execute_next_task,
            change_webview_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
