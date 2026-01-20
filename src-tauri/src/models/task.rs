use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::brief_case::BriefCase;
use super::social_media::SocialMedia;

/// Represents a task that requires comments to be posted on a social media post
///
/// A Task consists of a link to a social media post and comments that need to be posted
/// from different user accounts (BriefCases). When a Task is created, it automatically
/// finds and assigns all BriefCases that match its social media platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier for this task
    pub task_id: Uuid,
    /// The URL of the social media post where comments should be posted
    pub link: String,
    /// The list of comments/messages to be posted (one comment per BriefCase)
    pub comments: Vec<String>,
    /// The social media platform this task is for
    pub social_media: SocialMedia,
    /// All BriefCases (user accounts) that can post comments for this task
    pub related_brief_cases: Vec<BriefCase>,
    /// Current position in the comments array
    pub comment_index: usize,
}

impl Task {
    /// Creates a new Task and assigns relevant BriefCases based on social media platform
    ///
    /// This constructor automatically finds all BriefCases that match the specified
    /// social_media platform and assigns them to the task.
    ///
    /// # Arguments
    /// * `link` - URL of the social media post
    /// * `comments` - List of comment messages to post (formatted)
    /// * `social_media` - The platform this task targets
    /// * `all_brief_cases` - All available BriefCases in the system
    ///
    /// # Returns
    /// A new Task with matching BriefCases assigned
    pub fn new(
        link: String,
        comments: Vec<String>,
        social_media: SocialMedia,
        all_brief_cases: &[BriefCase],
    ) -> Self {
        // Filter and collect all BriefCases matching this task's social media platform
        let related_brief_cases: Vec<BriefCase> = all_brief_cases
            .iter()
            .filter(|bc| bc.social_media == social_media)
            .cloned()
            .collect();

        Self {
            task_id: Uuid::new_v4(),
            link,
            comments,
            social_media,
            related_brief_cases,
            comment_index: 0,
        }
    }

    /// Formats raw comment text into a list of clean comments
    ///
    /// Handles bullet points, numbered lists, and removes empty lines and whitespace.
    ///
    /// # Arguments
    /// * `input` - Raw multi-line comment text
    ///
    /// # Returns
    /// A vector of formatted, non-empty comment strings
    pub fn format_comments(input: &str) -> Vec<String> {
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                // Remove bullet points (*, -, •) from the start
                let mut cleaned =
                    line.trim_start_matches(|c: char| c == '*' || c == '-' || c == '•');

                // Handle numbered lists (e.g., "1. ", "20) ")
                // Find first character that isn't a digit, dot, parenthesis, or space
                if let Some(first_char_index) =
                    cleaned.find(|c: char| !c.is_ascii_digit() && c != '.' && c != ')' && c != ' ')
                {
                    cleaned = &cleaned[first_char_index..];
                }

                // Final trim to catch any leftover spaces
                let final_str = cleaned.trim().to_string();

                if final_str.is_empty() {
                    None
                } else {
                    Some(final_str)
                }
            })
            .collect()
    }

    /// Gets the total number of BriefCases assigned to this task
    ///
    /// # Returns
    /// The count of BriefCases that can post comments for this task
    pub fn briefcase_count(&self) -> usize {
        self.related_brief_cases.len()
    }
}
