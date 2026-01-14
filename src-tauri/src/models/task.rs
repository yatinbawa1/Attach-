use regex::Regex;
use serde::{Deserialize, Serialize};
use std::ops::AddAssign;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    link: String,
    comments: Vec<String>,
    comment_unformatted: String,
    comment_counter: usize,
    task_id: Uuid,
}

impl Task {
    pub fn format_comments(string: &String) -> Vec<String> {
        let re_leading = Regex::new(r"^[ \t]*(\d+\.|\*|>)[ \t]*").unwrap();
        let re_trailing = Regex::new(r"[ \t]*\[\d+\][ \t]*$").unwrap();

        string
            .lines()
            .map(|line| {
                let cleaned_start = re_leading.replace(line, "");
                let cleaned_both = re_trailing.replace(&cleaned_start, "");
                cleaned_both.trim().to_string()
            })
            .filter(|s| !s.is_empty())
            .collect()
    }

    pub fn new(link: String, comment_unformatted: String) -> Self {
        Self {
            link,
            task_id: Uuid::new_v4(),
            comments: Self::format_comments(&comment_unformatted),
            comment_unformatted,
            comment_counter: 0,
        }
    }
}

// Increase comment counter without another function.
impl AddAssign<usize> for Task {
    fn add_assign(&mut self, rhs: usize) {
        self.comment_counter += rhs
    }
}
