use crate::components::editor::Editor;
use crate::components::outline::Outline;
use crate::components::preview::Preview;
use crate::components::quick_switcher::QuickSwitcher;
use crate::components::sidebar::Sidebar;
use crate::components::status_bar::StatusBar;
use crate::components::toast::Toast;
use crate::state::{AppState, SaveStatus};
use dioxus::document::eval;
use dioxus::prelude::*;
use std::time::Duration;

#[component]
pub fn App() -> Element {
    let mut state = use_signal(AppState::new);
    let mut debounce_timer = use_signal(|| 0u32);
    let mut is_quick_switcher_open = use_signal(|| false);
    let mut toast_timer = use_signal(|| 0u32);
    let mut jump_to_line = use_signal(|| None::<usize>);
    let mut font_size = use_signal(|| 20u8);

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

    // Auto-dismiss toast after 5 seconds
    let deleted_note = (state.read().deleted_note)();
    let deleted_note_for_effect = deleted_note.clone();
    use_effect(move || {
        if deleted_note_for_effect.is_some() {
            let timer_id = toast_timer() + 1;
            toast_timer.set(timer_id);
            spawn(async move {
                async_std::task::sleep(Duration::from_secs(5)).await;
                if toast_timer() == timer_id {
                    state.write().clear_deleted_note();
                }
            });
        }
    });

    let state_read = state.read();
    let notes = (state_read.notes)();
    let current_note = (state_read.current_note)();
    let is_sidebar_visible = (state_read.is_sidebar_visible)();
    let is_preview_visible = (state_read.is_preview_visible)();
    let is_focus_mode = (state_read.is_focus_mode)();
    let save_status = (state_read.save_status)();
    drop(state_read);

    let current_id = current_note.as_ref().map(|n| n.id.clone());
    let content = current_note
        .as_ref()
        .map(|n| n.content.clone())
        .unwrap_or_default();
    let has_note = current_note.is_some();

    let app_class = if is_focus_mode {
        "app-container focus-mode"
    } else {
        "app-container"
    };

    rsx! {
        div {
            class: "{app_class}",
            tabindex: "0",
            onkeydown: move |evt| {
                if evt.modifiers().ctrl() || evt.modifiers().meta() {
                    if evt.modifiers().shift() {
                        match evt.key() {
                            Key::Character(c) if c == "F" || c == "f" => {
                                evt.prevent_default();
                                state.write().toggle_focus_mode();
                            }
                            _ => {}
                        }
                    } else {
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
                            Key::Character(c) if c == "k" => {
                                evt.prevent_default();
                                is_quick_switcher_open.set(!is_quick_switcher_open());
                            }
                            _ => {}
                        }
                    }
                }
            },

            div { class: "main-content",
                // Left: Document outline (always visible when note is open)
                if has_note {
                    Outline {
                        content: content.clone(),
                        on_jump: move |line: usize| {
                            jump_to_line.set(Some(line));
                        },
                    }
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
                            jump_to_line: jump_to_line(),
                        }

                        if is_preview_visible {
                            Preview { content: content.clone() }
                        }
                    }
                } else {
                    div { class: "welcome",
                        h1 { "Mime" }
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

            // Bottom: Unified control bar
            StatusBar {
                content: if has_note { Some(content.clone()) } else { None },
                has_note,
                is_sidebar_visible,
                is_preview_visible,
                is_focus_mode,
                font_size: font_size(),
                on_toggle_sidebar: move |_| state.write().toggle_sidebar(),
                on_toggle_preview: move |_| state.write().toggle_preview(),
                on_toggle_focus: move |_| state.write().toggle_focus_mode(),
                on_delete: move |_| state.write().delete_current_note(),
                on_font_size_change: move |size: u8| {
                    font_size.set(size);
                    let js = format!(
                        "document.documentElement.style.setProperty('--font-size-editor', '{}px')",
                        size
                    );
                    eval(&js);
                },
            }

            // Quick Switcher modal
            if is_quick_switcher_open() {
                QuickSwitcher {
                    notes: notes.clone(),
                    on_select: move |id: String| {
                        state.write().select_note(&id);
                        is_quick_switcher_open.set(false);
                    },
                    on_close: move |_| is_quick_switcher_open.set(false),
                }
            }

            // Toast notification for deleted note
            if deleted_note.is_some() {
                Toast {
                    message: "Note deleted".to_string(),
                    on_undo: move |_| {
                        state.write().undo_delete();
                    },
                    on_dismiss: move |_| {
                        state.write().clear_deleted_note();
                    },
                }
            }
        }
    }
}
