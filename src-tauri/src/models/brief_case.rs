use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct BriefCase {
    pub id: Uuid,
    pub social_media: SocialMedia,
    pub profile_id: Uuid,
    pub user_name: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SocialMedia {
    Youtube,
    X,
    Instagram,
    Facebook,
}

impl BriefCase {
    pub fn new(social_media: SocialMedia, profile_id: Uuid, user_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            social_media,
            profile_id,
            user_name,
            is_active: false,
        }
    }
}
