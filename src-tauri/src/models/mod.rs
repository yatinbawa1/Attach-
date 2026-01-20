/// Core data models for the attache application
///
/// This module defines the fundamental data structures used throughout the application:
/// - SocialMedia: Enumeration of supported platforms
/// - Profile: Browser profile that holds multiple user accounts
/// - BriefCase: Social media user account belonging to a Profile
/// - Task: A social media post with comments to be posted

pub mod brief_case;
pub mod profile;
pub mod social_media;
pub mod task;

// Re-export commonly used types for convenience
pub use brief_case::BriefCase;
pub use profile::Profile;
pub use social_media::SocialMedia;
pub use task::Task;
