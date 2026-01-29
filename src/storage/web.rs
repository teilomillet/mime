use crate::state::note::{Note, NoteSummary};
use crate::storage::traits::StorageBackend;
use gloo_storage::{LocalStorage, Storage};

const NOTES_INDEX_KEY: &str = "argo_notes_index";

#[derive(Clone)]
pub struct WebStorage;

impl WebStorage {
    pub fn new() -> Self {
        Self
    }

    fn note_key(id: &str) -> String {
        format!("argo_note_{}", id)
    }

    fn load_index(&self) -> Vec<String> {
        LocalStorage::get(NOTES_INDEX_KEY).unwrap_or_default()
    }

    fn save_index(&self, ids: &[String]) {
        let _ = LocalStorage::set(NOTES_INDEX_KEY, ids);
    }
}

impl Default for WebStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl StorageBackend for WebStorage {
    fn list_notes(&self) -> Vec<NoteSummary> {
        let ids = self.load_index();
        let mut summaries: Vec<NoteSummary> = ids
            .iter()
            .filter_map(|id| {
                LocalStorage::get::<Note>(&Self::note_key(id))
                    .ok()
                    .map(|note| NoteSummary::from(&note))
            })
            .collect();

        // Sort by updated_at descending
        summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        summaries
    }

    fn load_note(&self, id: &str) -> Option<Note> {
        LocalStorage::get(&Self::note_key(id)).ok()
    }

    fn save_note(&self, note: &Note) -> Result<(), String> {
        // Update index if new note
        let mut ids = self.load_index();
        if !ids.contains(&note.id) {
            ids.push(note.id.clone());
            self.save_index(&ids);
        }

        LocalStorage::set(&Self::note_key(&note.id), note).map_err(|e| e.to_string())
    }

    fn delete_note(&self, id: &str) -> Result<(), String> {
        // Remove from index
        let mut ids = self.load_index();
        ids.retain(|i| i != id);
        self.save_index(&ids);

        LocalStorage::delete(&Self::note_key(id));
        Ok(())
    }
}
