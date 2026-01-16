use crate::storage::file_errors::ReadError;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigFile {
    pub profile_path: PathBuf,
    pub brief_case_path: PathBuf,
    pub settings: Vec<String>,
}

impl ConfigFile {
    pub async fn new() -> Result<Self, ReadError> {
        let config_path = Self::check_files().await?;

        let config_data_json = match fs::read_to_string(config_path).await {
            Err(e) => return Err(ReadError::Io(e)),
            Ok(s) => s,
        };

        let config: ConfigFile = serde_json::from_str(config_data_json.as_str())?;

        Ok(config)
    }

    // This checks if config already exists or not
    // it returns the path to config_file, after creating it, if it does not exist.
    async fn check_files() -> Result<PathBuf, ReadError> {
        let project_dir = ProjectDirs::from("", "", "attache").ok_or(ReadError::NoConfigDir)?;
        let base_dir = project_dir.config_dir();

        // Create all folder if not made yet
        let config_dir = project_dir.config_dir().to_path_buf();
        fs::create_dir_all(&config_dir).await?;

        let config_path = base_dir.join("config.json");
        let profile_path = base_dir.join("profile.json");
        let brief_case_path = base_dir.join("brief_cases.json");

        let default_config_file = format!(
            r#"{{
                "profile_path": "{}",
                "brief_case_path": "{}",
                "settings": []
            }}"#,
            profile_path.display(),
            brief_case_path.display()
        );

        if !config_path.exists() {
            fs::write(config_path.clone(), default_config_file).await?;
        }

        if !brief_case_path.exists() {
            fs::write(brief_case_path, "[]").await?;
        }

        if !profile_path.exists() {
            fs::write(profile_path.clone(), "[]").await?;
        }

        Ok(config_path)
    }
}
