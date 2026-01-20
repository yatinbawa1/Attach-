use crate::models::{BriefCase, Profile, SocialMedia, Task};
use crate::state::AppState;
use crate::storage::Storage;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};
use uuid::Uuid;

/// Size configuration for profile windows
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum WindowSize {
    /// Full screen size
    Full,
    /// Partial size (2/3 of screen width)
    Partial,
}

/// Result structure for task execution operations
#[derive(Debug, Clone, Serialize)]
pub struct ExecutionResult {
    /// Whether the task was completed successfully
    pub completed: bool,
    /// ID of the profile to use for this execution
    pub profile_id: Uuid,
    /// URL to load in the browser
    pub link: String,
    /// Whether to switch to a new profile
    pub should_change_profile: bool,
    /// Index of the current task
    pub task_index: usize,
    /// Current comment to post
    pub comment: String,
}

/// Data sent to the panel UI for display
#[derive(Debug, Clone, Serialize)]
pub struct PanelData {
    /// The current task being executed
    pub current_task: Option<Task>,
    /// The current profile being used
    pub current_profile: Option<Profile>,
    /// All briefcases in the system
    pub briefcases: Vec<BriefCase>,
    /// Total number of tasks
    pub total_tasks: usize,
    /// Index of the current task
    pub current_task_index: Option<usize>,
    /// Overall progress: (visited_count, total_count)
    pub overall_progress: (usize, usize),
    /// Progress for each task: (task_index, visited, total)
    pub task_progress: Vec<(usize, usize, usize)>,
    /// Current comment to post
    pub current_comment: Option<String>,
}

/// ==================== Profile Commands ====================
/// Creates a new profile and saves it to disk
#[tauri::command]
pub async fn create_profile(
    profile_name: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<Profile, String> {
    let profile = Profile::new(profile_name, &app)
        .await
        .map_err(|e| format!("Failed to create profile: {}", e))?;

    state.add_profile(profile.clone()).await;

    // Save to disk
    let profiles = state.get_profiles().await;
    Storage::write_profiles(&app, &profiles)
        .await
        .map_err(|e| format!("Failed to save profiles: {}", e))?;

    // Notify frontend
    app.emit("profiles-changed", ())
        .map_err(|e| format!("Failed to emit event: {}", e))?;

    Ok(profile)
}

/// Loads all profiles from the state
#[tauri::command]
pub async fn load_profiles(_state: State<'_, AppState>) -> Result<Vec<Profile>, String> {
    Ok(_state.get_profiles().await)
}

/// Saves profiles to disk
#[tauri::command]
pub async fn save_profiles(
    profiles: Vec<Profile>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.set_profiles(profiles.clone()).await;

    Storage::write_profiles(&app, &profiles)
        .await
        .map_err(|e| format!("Failed to save profiles: {}", e))?;

    app.emit("profiles-changed", ())
        .map_err(|e| format!("Failed to emit event: {}", e))?;

    Ok(())
}

/// ==================== BriefCase Commands ====================
/// Creates a new BriefCase and saves it to disk
#[tauri::command]
pub async fn create_brief_case(
    profile_id: Uuid,
    user_name: String,
    social_media: SocialMedia,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let brief_case = BriefCase::new(social_media, profile_id, user_name);

    state.add_brief_case(brief_case).await;

    let briefcases = state.get_brief_cases().await;
    Storage::write_briefcases(&app, &briefcases)
        .await
        .map_err(|e| format!("Failed to save briefcases: {}", e))?;

    app.emit("briefcases-changed", ())
        .map_err(|e| format!("Failed to emit event: {}", e))?;

    Ok(())
}

/// Loads all BriefCases from the state
#[tauri::command]
pub async fn load_briefcases(state: State<'_, AppState>) -> Result<Vec<BriefCase>, String> {
    Ok(state.get_brief_cases().await)
}

/// Saves BriefCases to disk
#[tauri::command]
pub async fn save_briefcases(
    briefcases: Vec<BriefCase>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.set_brief_cases(briefcases.clone()).await;

    Storage::write_briefcases(&app, &briefcases)
        .await
        .map_err(|e| format!("Failed to save briefcases: {}", e))?;

    app.emit("briefcases-changed", ())
        .map_err(|e| format!("Failed to emit event: {}", e))?;

    Ok(())
}

/// ==================== Batch Data Commands ====================
/// Saves both profiles and briefcases in one operation
#[tauri::command]
pub async fn save_all_data(
    profiles: Vec<Profile>,
    briefcases: Vec<BriefCase>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.set_profiles(profiles.clone()).await;
    state.set_brief_cases(briefcases.clone()).await;

    Storage::write_profiles(&app, &profiles)
        .await
        .map_err(|e| format!("Failed to save profiles: {}", e))?;

    Storage::write_briefcases(&app, &briefcases)
        .await
        .map_err(|e| format!("Failed to save briefcases: {}", e))?;

    app.emit("profiles-changed", ())
        .map_err(|e| format!("Failed to emit event: {}", e))?;

    app.emit("briefcases-changed", ())
        .map_err(|e| format!("Failed to emit event: {}", e))?;

    Ok(())
}

/// ==================== Window Management Commands ====================
/// Launches a profile window with the specified URL

#[tauri::command]
pub async fn create_window_sized(
    app_handle: AppHandle,
    profile: Profile,
    url: &str,
    size_type: WindowSize,
    close_previous_window: bool,
) -> Result<String, String> {
    let data_dir = profile
        .get_data_path(&app_handle)
        .ok_or("Failed to get profile data path")?;

    let window_label = format!("profile-{}", profile.profile_id);

    if close_previous_window {
        // Close all profile windows first
        for (_, window) in app_handle.webview_windows() {
            let label = window.label();
            if label.starts_with("profile-") {
                let _ = window.close();
            }
        }
    }

    // Get monitor dimensions for window sizing
    let monitor = app_handle
        .primary_monitor()
        .map_err(|e| format!("Failed to get monitor: {}", e))?
        .ok_or("No monitor available")?;

    let screen_width = monitor.size().width as f64 / monitor.scale_factor();
    let screen_height = monitor.size().height as f64 / monitor.scale_factor();

    // Profile window takes up 2/3 of the screen
    let webview_width = (screen_width * 0.66).floor();
    let webview_height = screen_height;
    let x = 0.0;
    let y = 0.0;

    let webview =
        WebviewWindowBuilder::new(&app_handle, &window_label, WebviewUrl::App(url.into()))
            .title(&format!("Browser - {}", profile.profile_name));

    match size_type {
        WindowSize::Full => {
            webview
                .inner_size(
                    monitor.size().width as f64 / monitor.scale_factor(),
                    monitor.size().height as f64 / monitor.scale_factor(),
                )
                .data_directory(data_dir)
                .position(0.0, 0.0)
                .resizable(true)
                .build()
                .map_err(|e| format!("Failed to build window: {}", e))?;
        }

        WindowSize::Partial => {
            webview
                .inner_size(webview_width, webview_height)
                .position(x, y)
                .data_directory(data_dir)
                .resizable(true)
                .build()
                .map_err(|e| format!("Failed to build window: {}", e))?;
        }
    }

    Ok(window_label)
}

#[tauri::command]
pub async fn launch_profile_window(
    app_handle: AppHandle,
    profile: Profile,
    url: String,
) -> Result<String, String> {
    Ok(create_window_sized(app_handle, profile, url.as_str(), WindowSize::Partial, true).await?)
}

/// Launches the control panel window
#[tauri::command]
pub async fn launch_panel_window(app_handle: AppHandle) -> Result<(), String> {
    // Return early if panel is already open
    if let Some(existing_panel) = app_handle.get_webview_window("panel") {
        let _ = existing_panel.show();
        let _ = existing_panel.set_focus();
        return Ok(());
    }

    // Get monitor dimensions
    let monitor = app_handle
        .primary_monitor()
        .map_err(|e| format!("Failed to get monitor: {}", e))?
        .ok_or("No monitor available")?;

    let screen_width = monitor.size().width as f64 / monitor.scale_factor();
    let screen_height = monitor.size().height as f64 / monitor.scale_factor();

    // Panel takes up remaining 1/3 of screen
    let profile_width = (screen_width * 0.66).floor();
    let panel_width = screen_width - profile_width;
    let panel_height = screen_height;
    let x = profile_width;
    let y = 0.0;

    let panel = WebviewWindowBuilder::new(&app_handle, "panel", WebviewUrl::App("/panel".into()));

    panel
        .title("Control Panel")
        .inner_size(panel_width, panel_height)
        .position(x, y)
        .resizable(true)
        .decorations(true)
        .always_on_top(false)
        .build()
        .map_err(|e| format!("Failed to build panel: {}", e))?;

    Ok(())
}

/// Changes the URL of the currently active profile window
#[tauri::command]
pub async fn change_webview_url(
    app: AppHandle,
    state: State<'_, AppState>,
    url: String,
) -> Result<(), String> {
    let label = state
        .get_current_window_label()
        .await
        .ok_or("No active profile window")?;

    let webview = app
        .get_webview_window(&label)
        .ok_or_else(|| format!("Webview with label '{}' not found", label))?;

    webview
        .eval(&format!("window.location.href = '{}'", url))
        .map_err(|e| format!("Failed to change URL: {}", e))?;

    Ok(())
}

/// Closes the workspace and all associated windows
#[tauri::command]
pub async fn close_workspace(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    // Close all profile windows
    for (_, window) in app.webview_windows() {
        let label = window.label();
        if label.starts_with("profile-") {
            let _ = window.close();
        }
    }

    // Close panel window
    if let Some(panel) = app.get_webview_window("panel") {
        let _ = panel.close();
    }

    // Reset state
    state.clear_current_window_label().await;
    state.set_running(false).await;

    Ok(())
}

/// ==================== Automation Commands ====================
/// Starts the automation process with the given tasks
#[tauri::command]
pub async fn start_automation(
    app: AppHandle,
    state: State<'_, AppState>,
    tasks_json: String,
) -> Result<ExecutionResult, String> {
    let tasks: Vec<Task> =
        serde_json::from_str(&tasks_json).map_err(|e| format!("Failed to parse tasks: {}", e))?;

    let briefcases = state.get_brief_cases().await;

    if briefcases.is_empty() {
        return Err(
            "No briefcases found. Please add briefcases before starting automation.".to_string(),
        );
    }

    // Create tasks with associated briefcases
    let tasks_with_briefcases: Vec<Task> = tasks
        .into_iter()
        .map(|task| Task::new(task.link, task.comments, task.social_media, &briefcases))
        .collect();

    // Check if any tasks have matching briefcases
    if tasks_with_briefcases
        .iter()
        .all(|t| t.related_brief_cases.is_empty())
    {
        return Err("No briefcases match the task platforms. Please add briefcases for the social media platforms you want to automate.".to_string());
    }

    // Set tasks and create execution plan
    state.set_tasks(tasks_with_briefcases).await;
    state.set_running(true).await;

    // Get first execution step
    let result = execute_next_step(app.clone(), state.clone()).await?;

    if !result.completed {
        // Launch panel and first profile window
        launch_panel_window(app.clone()).await?;

        let profile = state
            .get_profile_by_id(result.profile_id)
            .await
            .ok_or_else(|| format!("Profile {} not found", result.profile_id))?;

        let label = launch_profile_window(app.clone(), profile, result.link.clone()).await?;
        state.set_current_window_label(label).await;
    }

    Ok(result)
}

/// Executes the next step in the automation sequence
#[tauri::command]
pub async fn execute_next_step(
    _app: AppHandle,
    state: State<'_, AppState>,
) -> Result<ExecutionResult, String> {
    if state.is_complete().await {
        return Ok(ExecutionResult {
            completed: true,
            profile_id: Uuid::nil(),
            link: String::new(),
            should_change_profile: false,
            task_index: 0,
            comment: String::new(),
        });
    }

    // Check if we need to change profile
    let should_change = state.should_change_profile().await;

    // Get the next execution step (this advances the plan)
    let step = state
        .next_execution_step()
        .await
        .ok_or("No more execution steps")?;

    // Get profile info
    let profile_id = step.profile_id;
    let task_index = step.task_index;
    let link = step.link;
    let comment = state
        .get_current_comment(task_index)
        .await
        .unwrap_or_default();

    Ok(ExecutionResult {
        completed: false,
        profile_id,
        link,
        should_change_profile: should_change,
        task_index,
        comment,
    })
}

/// Handles the "next" action from the user (arrow key or button)
#[tauri::command]
pub async fn next_execution(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<ExecutionResult, String> {
    // Get current step before advancing
    let current_step = state.current_step().await;

    // Mark current briefcase as visited and increment comment index
    if let Some(step) = current_step {
        if let Some(task) = state.get_task(step.task_index).await {
            if let Some(briefcase) = task.related_brief_cases.get(step.briefcase_index) {
                state.mark_briefcase_visited(briefcase.id).await;
            }
            state.increment_comment_index(step.task_index).await;
        }
    }

    // Execute next step
    let result = execute_next_step(app.clone(), state.clone()).await?;

    if result.completed {
        return Ok(result);
    }

    // Either change profile or change URL
    if result.should_change_profile {
        let profile = state
            .get_profile_by_id(result.profile_id)
            .await
            .ok_or_else(|| format!("Profile {} not found", result.profile_id))?;

        let label = launch_profile_window(app.clone(), profile, result.link.clone()).await?;
        state.set_current_window_label(label).await;
    } else {
        let link = result.link.clone();
        change_webview_url(app, state, link).await?;
    }

    Ok(result)
}

/// ==================== Data Query Commands ====================
/// Gets panel data for the UI
#[tauri::command]
pub async fn get_panel_data(state: State<'_, AppState>) -> Result<PanelData, String> {
    let profiles = state.get_profiles().await;
    let briefcases = state.get_brief_cases().await;
    let total_tasks = state.task_count().await;
    let current_task_index = state.current_task_index().await;
    let overall_progress = state.get_progress().await;
    let task_progress = state.get_task_progress().await;

    let current_task = if let Some(idx) = current_task_index {
        state.get_task(idx).await
    } else {
        None
    };

    let current_profile = if let Some(profile_id) = state.current_profile_id().await {
        profiles
            .iter()
            .find(|p| p.profile_id == profile_id)
            .cloned()
    } else {
        None
    };

    let current_comment = if let Some(idx) = current_task_index {
        state.get_current_comment(idx).await
    } else {
        None
    };

    Ok(PanelData {
        current_task,
        current_profile,
        briefcases,
        total_tasks,
        current_task_index,
        overall_progress,
        task_progress,
        current_comment,
    })
}

/// Sets the current comment index for a task
#[tauri::command]
pub async fn set_comment_index(
    state: State<'_, AppState>,
    task_index: usize,
    comment_index: usize,
) -> Result<(), String> {
    state
        .set_task_comment_index(task_index, comment_index)
        .await;
    Ok(())
}

/// Test command for debugging
#[tauri::command]
pub fn test_command() -> String {
    "test".to_string()
}
