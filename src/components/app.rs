use crate::components::editor::Editor;
use crate::components::outline::Outline;
use crate::components::preview::Preview;
use crate::components::sidebar::Sidebar;
use crate::components::status_bar::StatusBar;
use crate::components::toolbar::Toolbar;
use crate::state::{AppState, SaveStatus};
use dioxus::prelude::*;
use std::time::Duration;

#[component]
pub fn App() -> Element {
    let mut state = use_signal(AppState::new);
    let mut debounce_timer = use_signal(|| 0u32);

    // Auto-save effect with debounce
    let save_status = (state.read().save_status)();
    use_effect(move || {
        if save_status == SaveStatus::Modified {
            let timer_id = debounce_timer();
            spawn(async move {
                async_std::task::sleep(Duration::from_secs(1)).await;
                if debounce_timer() == timer_id {
                    state.write().save_current_note();
                }
            });
            debounce_timer.set(timer_id.wrapping_add(1));
        }
    });

    let state_read = state.read();
    let notes = (state_read.notes)();
    let current_note = (state_read.current_note)();
    let is_sidebar_visible = (state_read.is_sidebar_visible)();
    let is_preview_visible = (state_read.is_preview_visible)();
    let save_status = (state_read.save_status)();
    drop(state_read);

    let current_id = current_note.as_ref().map(|n| n.id.clone());
    let content = current_note
        .as_ref()
        .map(|n| n.content.clone())
        .unwrap_or_default();
    let has_note = current_note.is_some();

    rsx! {
        div {
            class: "app-container",
            tabindex: "0",
            onkeydown: move |evt| {
                if evt.modifiers().ctrl() || evt.modifiers().meta() {
                    match evt.key() {
                        Key::Character(c) if c == "n" => {
                            evt.prevent_default();
                            state.write().create_note();
                        }
                        Key::Character(c) if c == "b" => {
                            evt.prevent_default();
                            state.write().toggle_sidebar();
                        }
                        Key::Character(c) if c == "p" => {
                            evt.prevent_default();
                            state.write().toggle_preview();
                        }
                        _ => {}
                    }
                }
            },

            Toolbar {
                is_sidebar_visible,
                is_preview_visible,
                has_note,
                on_toggle_sidebar: move |_| state.write().toggle_sidebar(),
                on_toggle_preview: move |_| state.write().toggle_preview(),
                on_delete: move |_| state.write().delete_current_note(),
            }

            div { class: "main-content",
                // Left: Document outline (always visible when note is open)
                if has_note {
                    Outline { content: content.clone() }
                }

                // Center: Editor (and preview if enabled)
                if has_note {
                    div { class: "editor-area",
                        Editor {
                            content: content.clone(),
                            save_status,
                            on_change: move |new_content: String| {
                                state.write().update_content(new_content);
                            },
                        }

                        if is_preview_visible {
                            Preview { content: content.clone() }
                        }
                    }
                } else {
                    div { class: "welcome",
                        h1 { "Argo" }
                        p { "A distraction-free note-taking app" }
                        p { class: "hint",
                            "Press "
                            kbd { "Ctrl+N" }
                            " or click "
                            strong { "+" }
                            " to create a new note"
                        }
                    }
                }

                // Right: Notes list (only when visible)
                if is_sidebar_visible {
                    Sidebar {
                        notes: notes.clone(),
                        current_id: current_id.clone(),
                        on_select: move |id: String| state.write().select_note(&id),
                        on_new: move |_| state.write().create_note(),
                    }
                }
            }

            // Bottom: Status bar with timer (fades when not hovered)
            StatusBar {}
        }
    }
}
