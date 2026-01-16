use crate::models::brief_case::BriefCase;
use crate::models::profile::Profile;
use crate::models::task::Task;
use crate::models::AppItem;
use crate::storage::Storage;
use crate::ProjectErrors::project_errors::ReadError;
use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;

pub struct State {
    profiles: RwLock<Vec<Profile>>,
    brief_cases: RwLock<Vec<BriefCase>>,
    tasks: Vec<Task>,
    storage: Storage,
}

impl State {
    pub async fn new(mut storage: Storage) -> Result<Self, ReadError> {
        Ok(Self {
            profiles: RwLock::from(storage.read_profiles_from_disk().await?),
            brief_cases: RwLock::from(storage.read_brief_cases_from_disk().await?),
            tasks: Vec::new(),
            storage: storage,
        })
    }
}

// Helper to handle the "Update, Save, and Emit" logic
async fn update_and_persist<T: AppItem>(
    app: &AppHandle,
    lock_root: &RwLock<Vec<T>>,
    item: T,
    storage: &Storage,
    event_name: &str,
) -> Result<(), String> {
    // 1. Update memory
    let current_list = {
        let mut lock = lock_root.write().await;
        lock.push(item);
        lock.clone()
    };

    // 2. Persist to disk (Assuming storage has a generic method or handles T)
    // Note: You might need to adjust your Storage method to be generic too!
    match storage.write_to_disk(&current_list).await {
        Ok(_) => {
            app.emit(event_name, "success").map_err(|e| e.to_string())?;
            Ok(())
        }
        Err(err) => {
            let error_msg = format!("Error [Can Not Save] [{}]", err);
            app.emit(event_name, &error_msg).ok();
            Err(error_msg)
        }
    }
}

#[tauri::command]
pub async fn add_item_persist(
    app: tauri::AppHandle,
    state: tauri::State<'_, State>,
    item_type: String,          // "profile" or "briefcase"
    payload: serde_json::Value, // Generic JSON from frontend
) -> Result<(), String> {
    match item_type.as_str() {
        "profile" => {
            let item: Profile = serde_json::from_value(payload).map_err(|e| e.to_string())?;
            update_and_persist(
                &app,
                &state.profiles,
                item,
                &state.storage,
                "profile-status",
            )
            .await
        }
        "briefcase" => {
            let item: BriefCase = serde_json::from_value(payload).map_err(|e| e.to_string())?;
            update_and_persist(
                &app,
                &state.brief_cases,
                item,
                &state.storage,
                "briefcase-status",
            )
            .await
        }
        _ => Err("Unknown item type".to_string()),
    }
}
