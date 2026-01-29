use crate::state::note::{Note, NoteSummary, SaveStatus};
use crate::storage::{create_storage, PlatformStorage, StorageBackend};
use dioxus::prelude::*;

#[derive(Clone)]
pub struct AppState {
    pub notes: Signal<Vec<NoteSummary>>,
    pub current_note: Signal<Option<Note>>,
    pub deleted_note: Signal<Option<Note>>,
    pub is_sidebar_visible: Signal<bool>,
    pub is_outline_visible: Signal<bool>,
    pub is_preview_visible: Signal<bool>,
    pub is_focus_mode: Signal<bool>,
    pub save_status: Signal<SaveStatus>,
    storage: PlatformStorage,
}

impl AppState {
    pub fn new() -> Self {
        let storage = create_storage();
        let notes = storage.list_notes();

        Self {
            notes: Signal::new(notes),
            current_note: Signal::new(None),
            deleted_note: Signal::new(None),
            is_sidebar_visible: Signal::new(false),
            is_outline_visible: Signal::new(false),
            is_preview_visible: Signal::new(false),
            is_focus_mode: Signal::new(false),
            save_status: Signal::new(SaveStatus::Saved),
            storage,
        }
    }

    pub fn load_notes(&mut self) {
        let notes = self.storage.list_notes();
        self.notes.set(notes);
    }

    pub fn create_note(&mut self) {
        // Save current note before creating new one
        self.save_current_note();

        let note = Note::new();
        if self.storage.save_note(&note).is_ok() {
            let id = note.id.clone();
            self.load_notes();
            // Load directly to avoid double-save
            if let Some(new_note) = self.storage.load_note(&id) {
                self.current_note.set(Some(new_note));
                self.save_status.set(SaveStatus::Saved);
            }
        }
    }

    pub fn select_note(&mut self, id: &str) {
        // Save current note before switching
        self.save_current_note();

        if let Some(note) = self.storage.load_note(id) {
            self.current_note.set(Some(note));
            self.save_status.set(SaveStatus::Saved);
        }
    }

    pub fn update_content(&mut self, content: String) {
        if let Some(mut note) = (self.current_note)() {
            note.content = content;
            note.extract_title();
            note.touch();
            self.current_note.set(Some(note));
            self.save_status.set(SaveStatus::Modified);
        }
    }

    pub fn save_current_note(&mut self) {
        if let Some(note) = (self.current_note)() {
            self.save_status.set(SaveStatus::Saving);
            if self.storage.save_note(&note).is_ok() {
                self.save_status.set(SaveStatus::Saved);
                self.load_notes();
            } else {
                self.save_status.set(SaveStatus::Error);
            }
        }
    }

    pub fn delete_current_note(&mut self) {
        if let Some(note) = (self.current_note)() {
            if self.storage.delete_note(&note.id).is_ok() {
                self.deleted_note.set(Some(note));
                self.current_note.set(None);
                self.load_notes();

                // Select first note if available
                let notes = (self.notes)();
                if let Some(first) = notes.first() {
                    self.select_note(&first.id);
                }
            }
        }
    }

    pub fn undo_delete(&mut self) {
        if let Some(note) = (self.deleted_note)() {
            if self.storage.save_note(&note).is_ok() {
                self.deleted_note.set(None);
                self.load_notes();
                self.select_note(&note.id);
            }
        }
    }

    pub fn clear_deleted_note(&mut self) {
        self.deleted_note.set(None);
    }

    pub fn toggle_sidebar(&mut self) {
        let current = (self.is_sidebar_visible)();
        self.is_sidebar_visible.set(!current);
    }

    pub fn toggle_outline(&mut self) {
        let current = (self.is_outline_visible)();
        self.is_outline_visible.set(!current);
    }

    pub fn toggle_preview(&mut self) {
        let current = (self.is_preview_visible)();
        self.is_preview_visible.set(!current);
    }

    pub fn toggle_focus_mode(&mut self) {
        let current = (self.is_focus_mode)();
        self.is_focus_mode.set(!current);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
