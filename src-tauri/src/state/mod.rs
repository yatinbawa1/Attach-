use crate::models::task::Task;
use crate::models::{brief_case::BriefCase, profile::Profile};
use crate::project_errors::{ReadError, WriteError};
use crate::storage::Storage;
use crate::workspace::WorkspaceState;
use tauri::{Manager, Runtime};
use tokio::sync::RwLock;

pub struct AppState {
    pub brief_cases: RwLock<Vec<BriefCase>>,
    pub profiles: RwLock<Vec<Profile>>,
    pub workspace_state: RwLock<Option<WorkspaceState>>,
}

impl AppState {
    pub async fn new<R: Runtime, M: Manager<R>>(manager: &M) -> Result<Self, ReadError> {
        Storage::check_files(manager).await?;
        Ok(Self {
            brief_cases: Storage::read_from_disk(manager).await?.into(),
            profiles: Storage::read_from_disk(manager).await?.into(),
            workspace_state: None.into(),
        })
    }

    pub async fn add_brief_case(&self, brief_case: BriefCase) {
        self.brief_cases.write().await.push(brief_case);
    }

    pub async fn add_profile(&self, profile: Profile) {
        self.profiles.write().await.push(profile);
    }
    pub async fn save_profiles<R: Runtime, M: Manager<R>>(
        &self,
        app_handle: &M,
    ) -> Result<(), WriteError> {
        Storage::write_to_disk(app_handle, &self.profiles.read().await.clone()).await?;
        Ok(())
    }

    pub async fn save_brief_cases<R: Runtime, M: Manager<R>>(
        &self,
        app_handle: &M,
    ) -> Result<(), WriteError> {
        println!("Brief cases: {:?}", self.brief_cases.read().await);
        Storage::write_to_disk(app_handle, &self.brief_cases.read().await.clone()).await?;
        Ok(())
    }

    pub async fn get_profiles(&self) -> Vec<Profile> {
        self.profiles.read().await.clone()
    }

    pub async fn get_brief_cases(&self) -> Vec<BriefCase> {
        self.brief_cases.read().await.clone()
    }
    pub async fn create_workspace<R: Runtime, M: Manager<R>>(
        &self,
        manager: &M,
        tasks: Vec<Task>,
    ) -> Result<(), String> {
        let mut workspace = self.workspace_state.write().await;
        *workspace = Some(WorkspaceState::new(tasks));
        Ok(())
    }
}
