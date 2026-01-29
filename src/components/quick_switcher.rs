use crate::state::note::NoteSummary;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[component]
pub fn QuickSwitcher(
    notes: Vec<NoteSummary>,
    on_select: EventHandler<String>,
    on_close: EventHandler<()>,
) -> Element {
    let mut query = use_signal(String::new);
    let mut selected_index = use_signal(|| 0usize);

    let q = query().to_lowercase();
    let filtered_notes: Vec<NoteSummary> = if q.is_empty() {
        notes.clone()
    } else {
        notes
            .iter()
            .filter(|n| fuzzy_match(&n.title.to_lowercase(), &q))
            .cloned()
            .collect()
    };

    let filtered_len = filtered_notes.len();
    let current_selected = selected_index().min(filtered_len.saturating_sub(1));

    rsx! {
        div {
            class: "quick-switcher",
            onclick: move |_| on_close.call(()),

            div {
                class: "quick-switcher-dialog",
                onclick: move |evt| evt.stop_propagation(),
                onkeydown: {
                    let filtered_notes = filtered_notes.clone();
                    move |evt: KeyboardEvent| {
                        match evt.key() {
                            Key::Escape => on_close.call(()),
                            Key::ArrowDown => {
                                if filtered_len > 0 {
                                    selected_index.set((current_selected + 1) % filtered_len);
                                }
                            }
                            Key::ArrowUp => {
                                if filtered_len > 0 {
                                    selected_index.set(
                                        if current_selected == 0 {
                                            filtered_len.saturating_sub(1)
                                        } else {
                                            current_selected - 1
                                        }
                                    );
                                }
                            }
                            Key::Enter => {
                                if let Some(note) = filtered_notes.get(current_selected) {
                                    on_select.call(note.id.clone());
                                }
                            }
                            _ => {}
                        }
                    }
                },

                input {
                    class: "quick-switcher-input",
                    r#type: "text",
                    placeholder: "Search notes...",
                    autofocus: true,
                    value: "{query}",
                    oninput: move |evt| {
                        query.set(evt.value());
                        selected_index.set(0);
                    },
                }

                div { class: "quick-switcher-results",
                    for (idx, note) in filtered_notes.iter().enumerate() {
                        div {
                            key: "{note.id}",
                            class: if idx == current_selected { "quick-switcher-item selected" } else { "quick-switcher-item" },
                            onclick: {
                                let id = note.id.clone();
                                move |_| on_select.call(id.clone())
                            },
                            span { class: "quick-switcher-title", "{note.title}" }
                            span { class: "quick-switcher-meta", "{format_relative_time(note.updated_at)}" }
                        }
                    }
                    if filtered_notes.is_empty() {
                        div { class: "quick-switcher-empty", "No notes found" }
                    }
                }
            }
        }
    }
}

fn fuzzy_match(text: &str, query: &str) -> bool {
    let mut text_chars = text.chars().peekable();
    for query_char in query.chars() {
        loop {
            match text_chars.next() {
                Some(c) if c == query_char => break,
                Some(_) => continue,
                None => return false,
            }
        }
    }
    true
}

fn format_relative_time(dt: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(dt);

    if duration.num_minutes() < 1 {
        "just now".to_string()
    } else if duration.num_minutes() < 60 {
        format!("{}m ago", duration.num_minutes())
    } else if duration.num_hours() < 24 {
        format!("{}h ago", duration.num_hours())
    } else if duration.num_days() == 1 {
        "yesterday".to_string()
    } else if duration.num_days() < 7 {
        format!("{}d ago", duration.num_days())
    } else if duration.num_weeks() < 4 {
        format!("{}w ago", duration.num_weeks())
    } else {
        dt.format("%b %d").to_string()
    }
}
