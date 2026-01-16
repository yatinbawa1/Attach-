use crate::models::brief_case::BriefCase;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct Profile {
    pub profile_id: Uuid,
    pub profile_name: String,
    pub profile_path: PathBuf,
}

impl Profile {
    pub fn get_all_brief_case_ids(&self, all_brief_cases: &Vec<BriefCase>) -> Vec<Uuid> {
        all_brief_cases
            .iter()
            .filter(|brief_case| brief_case.profile_id == self.profile_id)
            .map(|brief_case| brief_case.id)
            .collect()
    }

    pub fn new(profile_name: String, profile_path: PathBuf) -> Self {
        Self {
            profile_id: Uuid::new_v4(),
            profile_name,
            profile_path,
        }
    }
}
