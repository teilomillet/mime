use crate::state::note::{Note, NoteSummary};
use crate::storage::traits::StorageBackend;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct DesktopStorage {
    notes_dir: PathBuf,
}

impl DesktopStorage {
    pub fn new() -> Self {
        let notes_dir = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".argo")
            .join("notes");

        // Ensure directory exists
        if !notes_dir.exists() {
            fs::create_dir_all(&notes_dir).ok();
        }

        Self { notes_dir }
    }

    fn note_path(&self, id: &str) -> PathBuf {
        self.notes_dir.join(format!("{}.json", id))
    }
}

impl Default for DesktopStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl StorageBackend for DesktopStorage {
    fn list_notes(&self) -> Vec<NoteSummary> {
        let mut summaries = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.notes_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(note) = serde_json::from_str::<Note>(&content) {
                            summaries.push(NoteSummary::from(&note));
                        }
                    }
                }
            }
        }

        // Sort by updated_at descending
        summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        summaries
    }

    fn load_note(&self, id: &str) -> Option<Note> {
        let path = self.note_path(id);
        fs::read_to_string(path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
    }

    fn save_note(&self, note: &Note) -> Result<(), String> {
        let path = self.note_path(&note.id);
        let content = serde_json::to_string_pretty(note).map_err(|e| e.to_string())?;
        fs::write(path, content).map_err(|e| e.to_string())
    }

    fn delete_note(&self, id: &str) -> Result<(), String> {
        let path = self.note_path(id);
        if path.exists() {
            fs::remove_file(path).map_err(|e| e.to_string())
        } else {
            Ok(())
        }
    }
}
