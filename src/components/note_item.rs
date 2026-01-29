use crate::state::NoteSummary;
use chrono::{DateTime, Utc};
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
    let date = format_relative_time(note.updated_at);

    rsx! {
        div {
            class: class_name,
            onclick: move |_| on_click.call(id.clone()),
            div { class: "note-title", {title} }
            div { class: "note-date", {date} }
        }
    }
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
    } else {
        dt.format("%b %d").to_string()
    }
}
