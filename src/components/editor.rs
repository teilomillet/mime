use crate::state::SaveStatus;
use dioxus::document::eval;
use dioxus::prelude::*;

const LINE_HEIGHT_PX: f64 = 24.0;

#[component]
pub fn Editor(
    content: String,
    save_status: SaveStatus,
    on_change: EventHandler<String>,
    #[props(default)] jump_to_line: Option<usize>,
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

    use_effect(move || {
        if let Some(line) = jump_to_line {
            let scroll_pos = (line as f64) * LINE_HEIGHT_PX;
            let js = format!(
                r#"
                const textarea = document.querySelector('.editor-textarea');
                if (textarea) {{
                    textarea.scrollTop = {scroll_pos};
                }}
                "#
            );
            eval(&js);
        }
    });

    rsx! {
        div { class: "editor",
            textarea {
                class: "editor-textarea",
                placeholder: "Start writing...",
                value: "{content}",
                oninput: move |evt| on_change.call(evt.value()),
                onkeydown: move |evt: KeyboardEvent| {
                    if evt.modifiers().ctrl() || evt.modifiers().meta() {
                        match evt.key() {
                            Key::Character(c) if c == "b" || c == "B" => {
                                evt.prevent_default();
                                let new_content = format!("{}****", content);
                                on_change.call(new_content);
                            }
                            Key::Character(c) if c == "i" || c == "I" => {
                                evt.prevent_default();
                                let new_content = format!("{}**", content);
                                on_change.call(new_content);
                            }
                            Key::Character(c) if c == "k" || c == "K" => {
                                evt.prevent_default();
                                let new_content = format!("{}[](url)", content);
                                on_change.call(new_content);
                            }
                            _ => {}
                        }
                    }
                },
            }
            div { class: "editor-status {status_class}",
                "{status_text}"
            }
        }
    }
}
