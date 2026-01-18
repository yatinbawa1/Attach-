use crate::models::brief_case::BriefCase;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{Manager, Runtime};
use tokio::fs;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct Profile {
    pub profile_id: Uuid,
    pub profile_name: String,
}

impl Profile {
    pub fn get_all_brief_case_ids(&self, all_brief_cases: &Vec<BriefCase>) -> Vec<Uuid> {
        all_brief_cases
            .iter()
            .filter(|brief_case| brief_case.profile_id == self.profile_id)
            .map(|brief_case| brief_case.id)
            .collect()
    }

    pub async fn new<R: Runtime, M: Manager<R>>(
        profile_name: String,
        manager: &M,
    ) -> Result<Self, std::io::Error> {
        let profile_id = Uuid::new_v4();
        let base_dir = manager.path().app_data_dir();
        let profile_path = base_dir.unwrap().join(format!("profiles/{}", profile_id));
        fs::create_dir_all(&profile_path).await?;

        Ok(Self {
            profile_id,
            profile_name,
        })
    }
}
