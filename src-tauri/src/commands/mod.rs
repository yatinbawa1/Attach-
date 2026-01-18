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

#[tauri::command]
pub async fn close_workspace(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(webview) = app.get_webview_window("webview") {
        let _ = webview.close();
    }

    if let Some(panel) = app.get_webview_window("panel") {
        let _ = panel.close();
    }

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
pub fn launch_profile_window(
    app_handle: AppHandle,
    app_state: tauri::State<'_, AppState>,
    profile: Profile,
    url: String,
) -> Result<(), String> {
    let profile_dir = app_handle
        .path()
        .app_data_dir()
        .unwrap()
        .join(format!("profiles/{}", profile.profile_id));

    let webview = WebviewWindowBuilder::new(
        &app_handle,
        profile.profile_name.clone(),
        WebviewUrl::App(url.into()),
    );

    webview
        .title(&format!("Browser - {}", profile.profile_name))
        .data_directory(profile_dir.clone())
        .build()
        .map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn open_workspace(
    app: AppHandle,
    state: State<'_, AppState>,
    _tasks: Vec<Task>,
) -> Result<(), String> {
    let test_profile = Profile::new("Yatin".to_string(), &app).await.unwrap();
    launch_profile_window(app, state, test_profile, "https://facebook.com".to_string())
        .expect("TODO: panic message");
    Ok(())
}
