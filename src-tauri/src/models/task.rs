use crate::models::brief_case::{BriefCase, SocialMedia};
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::{Manager, Runtime};
use tokio::io::AsyncReadExt;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    pub link: String,
    pub comments: Vec<String>,
    comment_unformatted: String,
    pub comment_counter: usize,
    pub progress: usize,
    pub task_id: Uuid,
    pub social_media: SocialMedia,
    pub related_brief_cases: Option<Vec<BriefCase>>,
}

impl Task {
    fn format_comments(input: &str) -> Vec<String> {
        input
            .lines()
            .map(|line| line.trim()) // 1. Remove surrounding whitespace
            .filter(|line| !line.is_empty()) // 2. Skip empty lines
            .filter_map(|line| {
                // 3. Remove leading bullets/formatting (*, -, •)
                let mut cleaned =
                    line.trim_start_matches(|c: char| c == '*' || c == '-' || c == '•');

                // 4. Handle numbered lists (e.g., "1. ", "20) ")
                // We look for the first character that isn't a digit, dot, or parenthesis
                if let Some(first_char_index) =
                    cleaned.find(|c: char| !c.is_ascii_digit() && c != '.' && c != ')' && c != ' ')
                {
                    cleaned = &cleaned[first_char_index..];
                }

                // 5. Final trim to catch any spaces left behind after removing numbers/stars
                let final_str = cleaned.trim().to_string();

                if final_str.is_empty() {
                    None
                } else {
                    Some(final_str)
                }
            })
            .collect()
    }

    pub async fn new<R: Runtime, M: Manager<R>>(
        link: String,
        comment_unformatted: String,
        social_media: SocialMedia,
        app: M,
    ) -> Self {
        let brief_cases: Vec<BriefCase> = app.state::<AppState>().brief_cases.read().await.clone();
        let filtered_brief_cases: Vec<BriefCase> = brief_cases
            .into_iter()
            .filter(|bc| bc.social_media == social_media)
            .collect();

        Self {
            link,
            task_id: Uuid::new_v4(),
            comments: Self::format_comments(&comment_unformatted),
            comment_unformatted,
            comment_counter: 0,
            progress: 0,
            social_media,
            related_brief_cases: Some(filtered_brief_cases),
        }
    }
}
