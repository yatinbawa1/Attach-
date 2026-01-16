use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct BriefCase {
    pub id: Uuid,
    pub name: String,
    pub platform: SocialMedia,
    pub profile_id: Uuid,
    pub user_name: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SocialMedia {
    Youtube,
    X,
    Instagram,
    Facebook,
}

impl BriefCase {
    pub fn new(name: String, platform: SocialMedia, profile_id: Uuid, user_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            platform,
            profile_id,
            user_name,
        }
    }
}
