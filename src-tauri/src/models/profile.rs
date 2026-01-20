use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{Manager, Runtime};
use tokio::fs;
use uuid::Uuid;

/// Represents a browser profile that can hold multiple social media user accounts (BriefCases)
///
/// A Profile maps to a browser data directory that contains user session data.
/// Each Profile can have one BriefCase per social media platform.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Profile {
    /// Unique identifier for this profile
    pub profile_id: Uuid,
    /// Human-readable name for this profile
    pub profile_name: String,
}

impl Profile {
    /// Creates a new Profile with a unique ID and creates its data directory
    ///
    /// # Arguments
    /// * `profile_name` - The display name for this profile
    /// * `manager` - The Tauri app manager for accessing the app data directory
    ///
    /// # Returns
    /// A new Profile instance with the directory created on disk
    ///
    /// # Errors
    /// Returns an error if directory creation fails
    pub async fn new<R: Runtime, M: Manager<R>>(
        profile_name: String,
        manager: &M,
    ) -> Result<Self, std::io::Error> {
        let profile_id = Uuid::new_v4();
        
        // Get the app data directory and create the profile folder
        let app_data_dir = manager
            .path()
            .app_data_dir()
            .map_err(|_| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Unable to resolve app data directory"
            ))?;
            
        let profile_path = app_data_dir.join(format!("profiles/{}", profile_id));
        fs::create_dir_all(&profile_path).await?;

        Ok(Self {
            profile_id,
            profile_name,
        })
    }

    /// Returns the file system path to this profile's data directory
    ///
    /// # Arguments
    /// * `manager` - The Tauri app manager for accessing the app data directory
    ///
    /// # Returns
    /// The PathBuf to the profile's data directory, or None if the directory cannot be determined
    pub fn get_data_path<R: Runtime, M: Manager<R>>(&self, manager: &M) -> Option<PathBuf> {
        manager
            .path()
            .app_data_dir()
            .ok()
            .map(|dir| dir.join(format!("profiles/{}", self.profile_id)))
    }
}
