use crate::models::AppItem;
use crate::project_errors::{ReadError, WriteError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{Manager, Runtime};
use tokio::fs;

const PROFILE_FILE_NAME: &str = "profile.json";
const BRIEFCASE_FILE_NAME: &str = "brief-case.json";
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Storage {}

impl Storage {
    pub async fn check_files<R: Runtime, M: Manager<R>>(manager: &M) -> Result<(), ReadError> {
        let project_dir: PathBuf = manager
            .path()
            .app_data_dir()
            .map_err(|err| ReadError::NotFound)?;
        let base_dir = project_dir.join("config-files");

        fs::create_dir_all(&base_dir).await?;

        let profile_path = base_dir.join(PROFILE_FILE_NAME);
        let brief_case_path = base_dir.join(BRIEFCASE_FILE_NAME);

        if !brief_case_path.exists() {
            fs::write(&brief_case_path, "[]").await?;
        }

        if !profile_path.exists() {
            fs::write(&profile_path, "[]").await?;
        }

        Ok(())
    }

    pub async fn read_from_disk<T: AppItem, R: Runtime, M: Manager<R>>(
        app: &M,
    ) -> Result<Vec<T>, ReadError> {
        let base_dir = app.path().app_data_dir().unwrap().join("config-files");

        match T::item_type() {
            "profile" => {
                let data = fs::read_to_string(base_dir.join(PROFILE_FILE_NAME)).await?;
                Ok(serde_json::from_str(&data)?)
            }
            "briefcase" => {
                let data = fs::read_to_string(base_dir.join(BRIEFCASE_FILE_NAME)).await?;
                Ok(serde_json::from_str(&data)?)
            }
            _ => Err(ReadError::AppItemTypeMismatch),
        }
    }

    pub async fn write_to_disk<T: AppItem, R: Runtime, M: Manager<R>>(
        app: &M,
        values: &Vec<T>,
    ) -> Result<(), WriteError> {
        let data = serde_json::to_string(&values)?;
        let base_dir = app.path().app_data_dir().unwrap().join("config-files");

        if !values.is_empty() {
            if T::item_type() == "profile" {
                fs::write(base_dir.join(PROFILE_FILE_NAME), data).await?;
                Ok(())
            } else if T::item_type() == "briefcase" {
                println!("Adding briefcase");
                fs::write(base_dir.join(BRIEFCASE_FILE_NAME), data).await?;
                Ok(())
            } else {
                Err(WriteError::AppItemTypeMismatch)
            }
        } else {
            Err(WriteError::EmptyValue)
        }
    }
}
