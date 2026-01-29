use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Note {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title: String::from("Untitled"),
            content: String::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn extract_title(&mut self) {
        // Extract title from first markdown heading or first line
        let title = self
            .content
            .lines()
            .find(|line| !line.trim().is_empty())
            .map(|line| {
                // Remove markdown heading prefix
                line.trim_start_matches('#').trim().to_string()
            })
            .filter(|t| !t.is_empty())
            .unwrap_or_else(|| String::from("Untitled"));

        // Truncate if too long
        self.title = if title.len() > 50 {
            format!("{}...", &title[..47])
        } else {
            title
        };
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

impl Default for Note {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoteSummary {
    pub id: String,
    pub title: String,
    pub updated_at: DateTime<Utc>,
}

impl From<&Note> for NoteSummary {
    fn from(note: &Note) -> Self {
        Self {
            id: note.id.clone(),
            title: note.title.clone(),
            updated_at: note.updated_at,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SaveStatus {
    #[default]
    Saved,
    Saving,
    Modified,
    Error,
}
