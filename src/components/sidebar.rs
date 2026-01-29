use crate::components::note_item::NoteItem;
use crate::state::NoteSummary;
use dioxus::prelude::*;

#[component]
pub fn Sidebar(
    notes: Vec<NoteSummary>,
    current_id: Option<String>,
    on_select: EventHandler<String>,
    on_new: EventHandler<()>,
) -> Element {
    let mut search_query = use_signal(String::new);

    let filtered_notes: Vec<_> = notes
        .iter()
        .filter(|note| {
            let query = search_query.read().to_lowercase();
            query.is_empty() || note.title.to_lowercase().contains(&query)
        })
        .cloned()
        .collect();

    let filtered_count = filtered_notes.len();
    let total_count = notes.len();
    let has_query = !search_query.read().is_empty();
    let is_empty = filtered_notes.is_empty();

    rsx! {
        aside { class: "sidebar",
            div { class: "sidebar-header",
                h2 { "Notes" }
                button {
                    class: "btn-new",
                    onclick: move |_| on_new.call(()),
                    title: "New Note (Ctrl+N)",
                    "+"
                }
            }
            div { class: "sidebar-search",
                input {
                    class: "sidebar-search-input",
                    r#type: "text",
                    placeholder: "Search notes...",
                    value: "{search_query}",
                    oninput: move |e| search_query.set(e.value()),
                }
                if has_query {
                    button {
                        class: "sidebar-search-clear",
                        onclick: move |_| search_query.set(String::new()),
                        "×"
                    }
                }
            }
            if has_query {
                div { class: "sidebar-search-count",
                    "{filtered_count} of {total_count} notes"
                }
            }
            div { class: "notes-list",
                for note in filtered_notes {
                    NoteItem {
                        key: "{note.id}",
                        note: note.clone(),
                        is_selected: current_id.as_ref() == Some(&note.id),
                        on_click: move |id| on_select.call(id),
                    }
                }
                if is_empty {
                    div { class: "empty-state",
                        if has_query {
                            "No matching notes"
                        } else {
                            "No notes yet"
                            br {}
                            "Click + to create one"
                        }
                    }
                }
            }
        }
    }
}
