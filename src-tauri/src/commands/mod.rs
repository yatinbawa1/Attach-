use crate::models::brief_case::{BriefCase, SocialMedia};
use crate::models::profile::Profile;
use crate::models::task::Task;
use crate::state::AppState;
use tauri::{AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};
use uuid::Uuid;

#[tauri::command]
pub fn test_command() -> String {
    "test".to_string()
}

fn normalize_name(name: String) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '/' || c == ':' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

#[tauri::command]
pub async fn close_workspace(
    app: AppHandle,
    state: State<'_, AppState>,
    profile_name: Option<String>,
) -> Result<(), String> {
    let profile_label = state.get_current_profile_window_label().await;

    if let Some(label) = profile_label {
        if let Some(webview) = app.get_webview_window(&label) {
            let _ = webview.close();
        }
    }

    if let Some(panel) = app.get_webview_window("panel") {
        let _ = panel.close();
    }

    let mut workspace = state.workspace_state.write().await;
    if let Some(ws) = workspace.as_mut() {
        ws.is_running = false;
    }
    *workspace = None;

    let mut execution_plan = state.execution_plan.write().await;
    *execution_plan = None;

    Ok(())
}

#[tauri::command]
pub async fn create_profile(
    profile_name: String,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<Profile, String> {
    let profile = Profile::new(profile_name, &app).await.unwrap();
    state.add_profile(profile.clone()).await;
    state.save_profiles(&app).await.map_err(|e| e.to_string())?; // saves

    // Emit event to notify frontend
    app.emit("profiles-changed", {})
        .map_err(|e| e.to_string())?;

    Ok(profile)
}

#[tauri::command]
pub async fn load_profiles(state: State<'_, AppState>) -> Result<Vec<Profile>, String> {
    let profiles = state.profiles.read().await;
    Ok(profiles.clone())
}

#[tauri::command]
pub async fn load_briefcases(state: State<'_, AppState>) -> Result<Vec<BriefCase>, String> {
    let briefcases = state.brief_cases.read().await;
    Ok(briefcases.clone())
}

#[tauri::command]
pub async fn save_profiles(
    profiles: Vec<Profile>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    *state.profiles.write().await = profiles;

    // Emit event to notify frontend
    app.emit("profiles-changed", {})
        .map_err(|e| e.to_string())?;

    state.save_profiles(&app).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn save_briefcases(
    briefcases: Vec<BriefCase>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    *state.brief_cases.write().await = briefcases;

    // Emit event to notify frontend
    app.emit("briefcases-changed", {})
        .map_err(|e| e.to_string())?;

    state
        .save_brief_cases(&app)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn save_all_data(
    profiles: Vec<Profile>,
    briefcases: Vec<BriefCase>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    *state.profiles.write().await = profiles;
    *state.brief_cases.write().await = briefcases;

    // Emit events to notify frontend
    app.emit("profiles-changed", {})
        .map_err(|e| e.to_string())?;
    app.emit("briefcases-changed", {})
        .map_err(|e| e.to_string())?;

    state.save_profiles(&app).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn create_brief_case(
    profile_id: Uuid,
    user_name: String,
    social_media: SocialMedia,
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let brief_case = BriefCase::new(social_media, profile_id, user_name);
    state.add_brief_case(brief_case).await;
    state.save_profiles(&app).await.map_err(|e| e.to_string())?; // saves

    // Emit event to notify frontend
    app.emit("briefcases-changed", {})
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn change_webview_url(
    app: AppHandle,
    state: State<'_, AppState>,
    url: &str,
) -> Result<(), String> {
    let label = state.get_current_profile_window_label().await;

    match label {
        Some(label) => {
            if let Some(webview) = app.get_webview_window(&label) {
                webview
                    .eval(&format!("window.location.href = '{}'", url))
                    .map_err(|e| e.to_string())?;
                Ok(())
            } else {
                Err(format!("Webview with label '{}' not found", label))
            }
        }
        None => Err("No profile window active".to_string()),
    }
}

#[tauri::command]
pub fn launch_profile_window(
    app_handle: AppHandle,
    app_state: State<'_, AppState>,
    profile: Profile,
    url: String,
) -> Result<String, String> {
    let profile_dir = app_handle
        .path()
        .app_data_dir()
        .unwrap()
        .join(format!("profiles/{}", profile.profile_id));

    let window_label = format!("profile-{}", profile.profile_id);

    if let Some(existing_webview) = app_handle.get_webview_window(&window_label) {
        let _ = existing_webview.close();
    }

    let monitor = app_handle
        .primary_monitor()
        .map_err(|e| e.to_string())?
        .ok_or("Failed to get primary monitor")?;

    let screen_width = monitor.size().width as f64;
    let screen_height = monitor.size().height as f64;

    let webview_width = (screen_width * 0.66).floor() / monitor.scale_factor();
    let webview_height = screen_height / monitor.scale_factor();
    let x = 0.0;
    let y = 0.0;

    let webview =
        WebviewWindowBuilder::new(&app_handle, &window_label, WebviewUrl::App(url.into()));

    webview
        .title(&format!("Browser - {}", profile.profile_name))
        .inner_size(webview_width, webview_height)
        .position(x, y)
        .data_directory(profile_dir.clone())
        .resizable(true)
        .build()
        .map_err(|err| err.to_string())?;

    Ok(window_label)
}

#[tauri::command]
async fn set_workspace_running(app_state: State<'_, AppState>) -> Result<(), String> {
    let mut workspace = app_state.workspace_state.write().await;
    if let Some(ws) = workspace.as_mut() {
        ws.is_running = true;
        Ok(())
    } else {
        Err("No workspace state available".to_string())
    }
}

#[derive(serde::Serialize, Clone)]
pub struct TaskExecutionResult {
    pub profile_id: Uuid,
    pub link: String,
    pub should_change_profile: bool,
    pub task_index: usize,
    pub completed: bool,
}

#[tauri::command]
pub async fn execute_next_task(state: State<'_, AppState>) -> Result<TaskExecutionResult, String> {
    let workspace = state.workspace_state.read().await;

    if let Some(ws) = workspace.as_ref() {
        if !ws.is_running {
            drop(workspace);
            set_workspace_running(state.clone()).await?;
        }
    } else {
        return Err("No workspace state available".to_string());
    }

    let execution_plan_lock = state.execution_plan.read().await;
    if execution_plan_lock.is_none() {
        drop(execution_plan_lock);
        let workspace = state.workspace_state.read().await;
        if let Some(ws) = workspace.as_ref() {
            state.create_execution_plan(&ws.task_list).await?;
        } else {
            return Err("No workspace state available".to_string());
        }
    }

    let should_change_profile = state.should_change_profile().await;

    if let Some(execution) = state.next_execution().await {
        Ok(TaskExecutionResult {
            profile_id: execution.profile_id,
            link: execution.link,
            should_change_profile,
            task_index: execution.task_index,
            completed: false,
        })
    } else {
        let mut execution_plan_lock = state.execution_plan.write().await;
        *execution_plan_lock = None;

        Ok(TaskExecutionResult {
            profile_id: Uuid::new_v4(),
            link: String::new(),
            should_change_profile: false,
            task_index: 0,
            completed: true,
        })
    }
}
#[derive(serde::Serialize)]
pub struct PanelData {
    pub current_task: Option<Task>,
    pub current_profile: Option<Profile>,
    pub briefcases: Vec<BriefCase>,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub visited_briefcases: usize,
    pub total_briefcases: usize,
    pub current_briefcase_visited: bool,
}

#[tauri::command]
pub async fn get_panel_data(state: State<'_, AppState>) -> Result<PanelData, String> {
    let profiles = state.profiles.read().await;
    let briefcases = state.brief_cases.read().await;
    let workspace_state = state.workspace_state.read().await;

    let (current_task, current_profile, total_tasks, visited, total, current_bc_visited) =
        if let Some(ws) = workspace_state.as_ref() {
            let task = ws.task_list.get(ws.current_task).cloned();

            let profile = ws.get_current_profile_id()
                .and_then(|profile_id| profiles.iter().find(|p| p.profile_id == profile_id))
                .cloned();

            let visited = ws.get_visited_count();
            let total = ws.get_total_briefcase_count();

            println!(
                "DEBUG PanelData - visited: {}, total: {}, task_progress: {}",
                visited,
                total,
                task.as_ref().map(|t| t.progress).unwrap_or(0)
            );

            let current_bc = ws.get_current_briefcase();
            let bc_visited = current_bc
                .map(|bc| ws.is_briefcase_visited(bc.id))
                .unwrap_or(false);

            (
                task,
                profile,
                ws.task_list.len(),
                visited,
                total,
                bc_visited,
            )
        } else {
            (None, None, 0, 0, 0, false)
        };

    Ok(PanelData {
        current_task,
        current_profile,
        briefcases: briefcases.clone(),
        total_tasks,
        completed_tasks: visited,
        visited_briefcases: visited,
        total_briefcases: total,
        current_briefcase_visited: current_bc_visited,
    })
}

#[tauri::command]
pub async fn launch_panel_window(app_handle: AppHandle) -> Result<(), String> {
    if let Some(existing_panel) = app_handle.get_webview_window("panel") {
        let _ = existing_panel.show();
        let _ = existing_panel.set_focus();
        return Ok(());
    }

    let monitor = app_handle
        .primary_monitor()
        .map_err(|e| e.to_string())?
        .ok_or("Failed to get primary monitor")?;

    let screen_width = monitor.size().width as f64 / monitor.scale_factor();
    let screen_height = monitor.size().height as f64 / monitor.scale_factor();

    let profile_width = (screen_width * 0.66).floor();
    let panel_width = screen_width - profile_width;
    let panel_height = screen_height;
    let x = profile_width + 5.0;
    let y = 0.0;

    println!(
        "Panel dimensions: {}x{}, position: ({}, {})",
        panel_width, panel_height, x, y
    );

    let panel = WebviewWindowBuilder::new(&app_handle, "panel", WebviewUrl::App("/panel".into()));

    let window = panel
        .title("Control Panel")
        .inner_size(panel_width, panel_height)
        .position(x, y)
        .resizable(true)
        .decorations(true)
        .always_on_top(false)
        .build()
        .map_err(|err| err.to_string())?;

    let _ = window.show();
    let _ = window.set_focus();

    println!("Panel window created successfully");

    Ok(())
}

#[tauri::command]
pub async fn start_automation(
    app: AppHandle,
    state: State<'_, AppState>,
    tasks: Vec<Task>,
) -> Result<TaskExecutionResult, String> {
    state.create_workspace(&app, tasks).await?;
    set_workspace_running(state.clone()).await?;

    let result = execute_next_task(state.clone()).await?;

    if !result.completed {
        launch_panel_window(app.clone()).await?;

        let profiles = state.profiles.read().await;
        let profile_to_use = profiles
            .iter()
            .find(|p| p.profile_id == result.profile_id)
            .cloned();
        drop(profiles);

        if let Some(profile) = profile_to_use {
            let label =
                launch_profile_window(app.clone(), state.clone(), profile, result.link.clone())?;
            state.set_current_profile_window_label(label).await;
        }
    }

    Ok(result)
}

#[tauri::command]
pub async fn open_workspace(
    app: AppHandle,
    state: State<'_, AppState>,
    tasks: Vec<Task>,
) -> Result<(), String> {
    state.create_workspace(&app, tasks).await?;
    set_workspace_running(state).await?;
    Ok(())
}

#[tauri::command]
pub async fn next_task_execution(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<TaskExecutionResult, String> {
    let result = state.advance_to_next_briefcase(&app).await?;

    if result.completed {
        return Ok(result);
    }

    if result.should_change_profile {
        let profiles = state.profiles.read().await;
        let profile_to_use = profiles
            .iter()
            .find(|p| p.profile_id == result.profile_id)
            .cloned();
        drop(profiles);

        if let Some(profile) = profile_to_use {
            let label =
                launch_profile_window(app.clone(), state.clone(), profile, result.link.clone())?;
            state.set_current_profile_window_label(label).await;
        }
    } else {
        change_webview_url(app, state, &result.link).await?;
    }

    Ok(result)
}

#[tauri::command]
pub async fn prev_task_execution(
    state: State<'_, AppState>,
) -> Result<TaskExecutionResult, String> {
    let mut workspace = state.workspace_state.write().await;

    if let Some(ws) = workspace.as_mut() {
        if ws.current_task > 0 {
            ws.current_task -= 1;
        }
        ws.is_running = true;

        let task = ws.task_list.get(ws.current_task).cloned();
        let execution_plan_lock = state.execution_plan.read().await;

        if let Some(plan) = execution_plan_lock.as_ref() {
            drop(execution_plan_lock);
            state.reset_execution_plan().await;
            let mut exec_plan = state.execution_plan.write().await;
            if let Some(existing_plan) = exec_plan.as_mut() {
                existing_plan.current_index = ws.current_task;
            }
        }

        Ok(TaskExecutionResult {
            profile_id: task
                .as_ref()
                .and_then(|t| {
                    t.related_brief_cases
                        .as_ref()?
                        .first()
                        .map(|bc| bc.profile_id)
                })
                .unwrap_or_default(),
            link: task.as_ref().map(|t| t.link.clone()).unwrap_or_default(),
            should_change_profile: false,
            task_index: ws.current_task,
            completed: false,
        })
    } else {
        Err("No workspace state available".to_string())
    }
}
