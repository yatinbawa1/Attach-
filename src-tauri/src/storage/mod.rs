mod config;
mod file_errors;

use crate::models::brief_case::BriefCase;
use crate::models::profile::Profile;
use crate::storage::config::ConfigFile;
use crate::storage::file_errors::ReadError;
use crate::storage::file_errors::WriteError;
use std::io::Stderr;

use directories::ProjectDirs;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::path::{Path, PathBuf};
use thiserror;
use thiserror::Error;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct Storage {
    pub config_file: ConfigFile,
    pub current_profiles: Option<Vec<Profile>>,
    pub current_brief_cases: Option<Vec<BriefCase>>,
}

impl Storage {
    pub async fn new() -> Result<Self, ReadError> {
        Ok(Self {
            config_file: ConfigFile::new().await?,
            current_profiles: None,
            current_brief_cases: None,
        })
    }
    
    async fn read_vec_from_json<T: DeserializeOwned, P: AsRef<Path>>(
        path: &P,
    ) -> Result<Vec<T>, ReadError> {
        let data = fs::read_to_string(path).await?;
        Ok(serde_json::from_str(&data)?)
    }

    async fn write_vec_to_json<T: Serialize, P: AsRef<Path>>(
        path: &P,
        values: &Vec<T>,
    ) -> Result<(), WriteError> {
        let data = serde_json::to_string(&values)?;
        fs::write(path, data).await?;
        Ok(())
    }
    pub async fn read_profiles_from_disk_into_current(&mut self) -> Result<(), ReadError> {
        self.current_profiles =
            Some(Self::read_vec_from_json(&self.config_file.profile_path).await?);
        Ok(())
    }

    pub async fn read_brief_cases_from_disk(&mut self) -> Result<(), ReadError> {
        self.current_brief_cases =
            Some(Self::read_vec_from_json(&self.config_file.brief_case_path).await?);
        Ok(())
    }

    pub async fn write_brief_cases_to_disk(&self, cases: &Vec<BriefCase>) -> Result<(), WriteError> {
        Ok(Self::write_vec_to_json(&self.config_file.brief_case_path, &cases).await?)
    }

    pub async fn write_profiles_to_disk(&self, profiles: &Vec<Profile>) -> Result<(), WriteError> {
        Ok(Self::write_vec_to_json(&self.config_file.profile_path, profiles).await?)
    }
}
