use crate::state::AppState;
/// Attache - Social Media Comment Automation Tool
///
/// This application automates posting comments on social media platforms
/// by managing multiple user accounts across different browser profiles.
///
/// # Core Concepts:
/// - **Profile**: A browser profile that can hold multiple social media user accounts
/// - **BriefCase**: A social media user account (one per platform per profile)
/// - **Task**: A social media post link with comments that need to be posted
/// - **Execution Plan**: Optimized sequence that groups work by profile to minimize switches
///
/// # Usage Flow:
/// 1. Create Profiles (browser sessions)
/// 2. Add BriefCases (user accounts) to each Profile
/// 3. Create Tasks with post links and comments
/// 4. Start automation - the system optimizes execution to minimize profile switches
/// 5. Navigate through tasks using arrow keys or next button
/// 6. Progress is tracked per-task and overall
use crate::storage::Storage;
use tauri::Manager;

// Re-export commands for use in invoke_handler
pub use crate::commands::*;

// Module declarations
mod commands;
mod execution;
mod models;
mod state;
mod storage;

/// Initializes and runs the Tauri application
///
/// Sets up the application state, initializes storage, registers commands,
/// and configures the Tauri builder with plugins and handlers.
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize storage and load data
            let storage_init =
                tauri::async_runtime::block_on(async { Storage::initialize(app).await });

            if let Err(e) = storage_init {
                eprintln!("Failed to initialize storage: {}", e);
                return Err(Box::new(e) as Box<dyn std::error::Error>);
            }

            // Load profiles and briefcases from disk
            let (profiles, briefcases) = tauri::async_runtime::block_on(async {
                let profiles: Vec<crate::models::Profile> =
                    Storage::read_profiles(app).await.unwrap_or_default();
                let briefcases: Vec<crate::models::BriefCase> =
                    Storage::read_briefcases(app).await.unwrap_or_default();
                (profiles, briefcases)
            });

            // Create and manage application state
            let state = AppState::new(profiles, briefcases);
            app.manage(state);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            // Profile management
            create_profile,
            load_profiles,
            save_profiles,
            // BriefCase management
            create_brief_case,
            load_briefcases,
            save_briefcases,
            // Batch operations
            save_all_data,
            // Window management
            launch_profile_window,
            launch_panel_window,
            change_webview_url,
            close_workspace,
            create_window_sized,
            // Automation
            start_automation,
            execute_next_step,
            next_execution,
            // Data queries
            get_panel_data,
            set_comment_index,
            // Test
            test_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
