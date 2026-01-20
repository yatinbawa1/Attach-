use std::path::PathBuf;
use tauri::{Manager, Runtime};
use tokio::fs;
use thiserror::Error;

/// File names for persisted data
const PROFILES_FILE: &str = "profiles.json";
const BRIEFCASES_FILE: &str = "briefcases.json";
const CONFIG_DIR: &str = "config";

/// Errors that can occur during storage operations
#[derive(Debug, Error)]
pub enum StorageError {
    /// File not found
    #[error("File not found")]
    #[allow(dead_code)]
    NotFound,
    
    /// I/O error occurred
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    /// JSON deserialization error
    #[error("Deserialization error: {0}")]
    Deserialize(#[from] serde_json::Error),
    
    /// Unable to resolve config directory
    #[error("Unable to resolve config directory")]
    NoConfigDir,
}

/// Storage manager for persisting application data to disk
///
/// Handles reading and writing Profiles and BriefCases to JSON files
/// in the application's data directory.
pub struct Storage;

impl Storage {
    /// Ensures the config directory and data files exist
    ///
    /// Creates the config directory and empty JSON files if they don't exist.
    ///
    /// # Arguments
    /// * `manager` - The Tauri app manager for accessing the app data directory
    ///
    /// # Returns
    /// Ok(()) if successful, Err(StorageError) if something fails
    pub async fn initialize<R: Runtime, M: Manager<R>>(manager: &M) -> Result<(), StorageError> {
        let config_dir = Self::get_config_path(manager)?;
        
        // Create config directory if it doesn't exist
        fs::create_dir_all(&config_dir).await?;
        
        // Create empty files if they don't exist
        let profiles_path = config_dir.join(PROFILES_FILE);
        let briefcases_path = config_dir.join(BRIEFCASES_FILE);
        
        if !profiles_path.exists() {
            fs::write(&profiles_path, "[]").await?;
        }
        
        if !briefcases_path.exists() {
            fs::write(&briefcases_path, "[]").await?;
        }
        
        Ok(())
    }

    /// Gets the path to the config directory
    ///
    /// # Arguments
    /// * `manager` - The Tauri app manager
    ///
    /// # Returns
    /// The PathBuf to the config directory
    fn get_config_path<R: Runtime, M: Manager<R>>(manager: &M) -> Result<PathBuf, StorageError> {
        let app_data_dir = manager
            .path()
            .app_data_dir()
            .map_err(|_| StorageError::NoConfigDir)?;
        
        Ok(app_data_dir.join(CONFIG_DIR))
    }

    /// Reads profiles from disk
    ///
    /// # Arguments
    /// * `manager` - The Tauri app manager
    ///
    /// # Returns
    /// A vector of Profile objects
    pub async fn read_profiles<R: Runtime, M: Manager<R>>(
        manager: &M,
    ) -> Result<Vec<crate::models::Profile>, StorageError> {
        let config_dir = Self::get_config_path(manager)?;
        let profiles_path = config_dir.join(PROFILES_FILE);
        
        let data = fs::read_to_string(&profiles_path).await?;
        let profiles: Vec<crate::models::Profile> = serde_json::from_str(&data)?;
        
        Ok(profiles)
    }

    /// Writes profiles to disk
    ///
    /// # Arguments
    /// * `manager` - The Tauri app manager
    /// * `profiles` - The vector of profiles to write
    ///
    /// # Returns
    /// Ok(()) if successful
    pub async fn write_profiles<R: Runtime, M: Manager<R>>(
        manager: &M,
        profiles: &[crate::models::Profile],
    ) -> Result<(), StorageError> {
        let config_dir = Self::get_config_path(manager)?;
        let profiles_path = config_dir.join(PROFILES_FILE);
        
        let data = serde_json::to_string_pretty(profiles)?;
        fs::write(&profiles_path, data).await?;
        
        Ok(())
    }

    /// Reads briefcases from disk
    ///
    /// # Arguments
    /// * `manager` - The Tauri app manager
    ///
    /// # Returns
    /// A vector of BriefCase objects
    pub async fn read_briefcases<R: Runtime, M: Manager<R>>(
        manager: &M,
    ) -> Result<Vec<crate::models::BriefCase>, StorageError> {
        let config_dir = Self::get_config_path(manager)?;
        let briefcases_path = config_dir.join(BRIEFCASES_FILE);
        
        let data = fs::read_to_string(&briefcases_path).await?;
        let briefcases: Vec<crate::models::BriefCase> = serde_json::from_str(&data)?;
        
        Ok(briefcases)
    }

    /// Writes briefcases to disk
    ///
    /// # Arguments
    /// * `manager` - The Tauri app manager
    /// * `briefcases` - The vector of briefcases to write
    ///
    /// # Returns
    /// Ok(()) if successful
    pub async fn write_briefcases<R: Runtime, M: Manager<R>>(
        manager: &M,
        briefcases: &[crate::models::BriefCase],
    ) -> Result<(), StorageError> {
        let config_dir = Self::get_config_path(manager)?;
        let briefcases_path = config_dir.join(BRIEFCASES_FILE);
        
        let data = serde_json::to_string_pretty(briefcases)?;
        fs::write(&briefcases_path, data).await?;
        
        Ok(())
    }
}
