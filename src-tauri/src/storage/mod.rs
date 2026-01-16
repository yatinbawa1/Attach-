mod config;
use crate::storage::config::ConfigFile;
use crate::project_errors::{ReadError, WriteError};

use crate::models::AppItem;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
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

    pub async fn read_from_disk<T: AppItem>(&self) -> Result<Vec<T>, ReadError> {
        match T::item_type() {
            "profile" => {
                let data = fs::read_to_string(&self.config_file.profile_path).await?;
                Ok(serde_json::from_str(&data)?)
            }
            "briefcase" => {
                let data = fs::read_to_string(&self.config_file.brief_case_path).await?;
                Ok(serde_json::from_str(&data)?)
            }
            _ => Err(ReadError::AppItemTypeMismatch),
        }
    }

    pub async fn write_to_disk<T: AppItem>(&self, values: &Vec<T>) -> Result<(), WriteError> {
        let data = serde_json::to_string(&values)?;
        if !values.is_empty() {
            if T::item_type() == "profile" {
                fs::write(&self.config_file.profile_path, data).await?;
                Ok(())
            } else if T::item_type() == "briefcase" {
                fs::write(&self.config_file.brief_case_path, data).await?;
                Ok(())
            } else {
                Err(WriteError::AppItemTypeMismatch)
            }
        } else {
            Err(WriteError::EmptyValue)
        }
    }
}
