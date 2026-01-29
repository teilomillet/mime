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
    let is_empty = notes.is_empty();

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
            div { class: "notes-list",
                for note in notes {
                    NoteItem {
                        key: "{note.id}",
                        note: note.clone(),
                        is_selected: current_id.as_ref() == Some(&note.id),
                        on_click: move |id| on_select.call(id),
                    }
                }
                if is_empty {
                    div { class: "empty-state",
                        "No notes yet"
                        br {}
                        "Click + to create one"
                    }
                }
            }
        }
    }
}
