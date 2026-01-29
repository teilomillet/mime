use crate::state::NoteSummary;
use dioxus::prelude::*;

#[component]
pub fn NoteItem(note: NoteSummary, is_selected: bool, on_click: EventHandler<String>) -> Element {
    let id = note.id.clone();
    let class_name = if is_selected {
        "note-item selected"
    } else {
        "note-item"
    };
    let title = note.title.clone();
    let date = note.updated_at.format("%b %d, %H:%M").to_string();

    rsx! {
        div {
            class: class_name,
            onclick: move |_| on_click.call(id.clone()),
            div { class: "note-title", {title} }
            div { class: "note-date", {date} }
        }
    }
}
