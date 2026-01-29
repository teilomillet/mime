use crate::markdown::render_markdown;
use dioxus::prelude::*;

#[component]
pub fn Preview(content: String) -> Element {
    let html = render_markdown(&content);

    rsx! {
        div { class: "preview",
            div {
                class: "preview-content",
                dangerous_inner_html: "{html}",
            }
        }
    }
}
