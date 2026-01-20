/// Represents a social media platform where tasks can be executed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SocialMedia {
    /// YouTube platform
    Youtube,
    /// X (formerly Twitter) platform
    X,
    /// Instagram platform
    Instagram,
    /// Facebook platform
    Facebook,
}

impl std::fmt::Display for SocialMedia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SocialMedia::Youtube => write!(f, "Youtube"),
            SocialMedia::X => write!(f, "X"),
            SocialMedia::Instagram => write!(f, "Instagram"),
            SocialMedia::Facebook => write!(f, "Facebook"),
        }
    }
}
