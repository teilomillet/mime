use crate::state::note::{Note, NoteSummary};

pub trait StorageBackend {
    fn list_notes(&self) -> Vec<NoteSummary>;
    fn load_note(&self, id: &str) -> Option<Note>;
    fn save_note(&self, note: &Note) -> Result<(), String>;
    fn delete_note(&self, id: &str) -> Result<(), String>;
}
