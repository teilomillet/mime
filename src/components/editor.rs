use crate::state::SaveStatus;
use dioxus::prelude::*;

#[component]
pub fn Editor(
    content: String,
    save_status: SaveStatus,
    on_change: EventHandler<String>,
) -> Element {
    let status_text = match save_status {
        SaveStatus::Saved => "Saved",
        SaveStatus::Saving => "Saving...",
        SaveStatus::Modified => "Modified",
        SaveStatus::Error => "Error",
    };

    let status_class = match save_status {
        SaveStatus::Saved => "status-saved",
        SaveStatus::Saving => "status-saving",
        SaveStatus::Modified => "status-modified",
        SaveStatus::Error => "status-error",
    };

    rsx! {
        div { class: "editor",
            textarea {
                class: "editor-textarea",
                placeholder: "Start writing...",
                value: "{content}",
                oninput: move |evt| on_change.call(evt.value()),
            }
            div { class: "editor-status {status_class}",
                "{status_text}"
            }
        }
    }
}
