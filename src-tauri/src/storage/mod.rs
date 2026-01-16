mod config;
use crate::models::brief_case::BriefCase;
use crate::models::profile::Profile;
use crate::storage::config::ConfigFile;
use crate::ProjectErrors::project_errors::{ReadError, WriteError};

use crate::models::AppItem;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct Storage {
    pub config_file: ConfigFile,
}

impl Storage {
    pub async fn new() -> Result<Self, ReadError> {
        Ok(Self {
            config_file: ConfigFile::new().await?,
        })
    }

    async fn read_vec_from_json<T: DeserializeOwned, P: AsRef<Path>>(
        path: &P,
    ) -> Result<Vec<T>, ReadError> {
        let data = fs::read_to_string(path).await?;
        Ok(serde_json::from_str(&data)?)
    }

    pub async fn write_to_disk<T: AppItem>(&self,values: &Vec<T>) -> Result<(), WriteError> {
        let data = serde_json::to_string(&values)?;
        if !values.is_empty() {
            if(values[0].item_type() == "profile") {
                fs::write(&self.config_file.profile_path, data).await?;
            }else if values[0].item_type() == "briefcase" {
                fs::write(&self.config_file.brief_case_path, data).await?;
            }else {
                panic!("invalid item , can only write profile and briefcase");
            }
        }
       
        Ok(())
    }

    pub async fn read_profiles_from_disk(&mut self) -> Result<Vec<Profile>, ReadError> {
        Ok(Self::read_vec_from_json(&self.config_file.profile_path).await?)
    }

    pub async fn read_brief_cases_from_disk(&mut self) -> Result<Vec<BriefCase>, ReadError> {
        Ok(Self::read_vec_from_json(&self.config_file.brief_case_path).await?)
    }
}
