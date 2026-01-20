use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::social_media::SocialMedia;

/// Represents a social media user account that can post comments
///
/// A BriefCase is essentially a user identity on a specific social media platform.
/// Each BriefCase belongs to a Profile (browser session) and can post comments on tasks.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct BriefCase {
    /// Unique identifier for this user account
    pub id: Uuid,
    /// The social media platform this account belongs to
    pub social_media: SocialMedia,
    /// The ID of the Profile (browser session) this account is associated with
    pub profile_id: Uuid,
    /// The username/display name for this account
    pub user_name: String,
}

impl BriefCase {
    /// Creates a new BriefCase user account
    ///
    /// # Arguments
    /// * `social_media` - The platform this account is for
    /// * `profile_id` - The Profile this account belongs to
    /// * `user_name` - The display name for this account
    ///
    /// # Returns
    /// A new BriefCase with a unique ID
    pub fn new(social_media: SocialMedia, profile_id: Uuid, user_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            social_media,
            profile_id,
            user_name,
        }
    }
}
