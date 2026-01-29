use dioxus::prelude::*;

#[component]
pub fn Toolbar(
    is_sidebar_visible: bool,
    is_preview_visible: bool,
    is_focus_mode: bool,
    has_note: bool,
    on_toggle_sidebar: EventHandler<()>,
    on_toggle_preview: EventHandler<()>,
    on_toggle_focus: EventHandler<()>,
    on_delete: EventHandler<()>,
) -> Element {
    let sidebar_class = if is_sidebar_visible { "btn-toolbar active" } else { "btn-toolbar" };
    let preview_class = if is_preview_visible { "btn-toolbar active" } else { "btn-toolbar" };
    let focus_class = if is_focus_mode { "btn-toolbar active" } else { "btn-toolbar" };

    rsx! {
        div { class: "toolbar",
            if has_note {
                button {
                    class: preview_class,
                    onclick: move |_| on_toggle_preview.call(()),
                    title: "Toggle Preview (Ctrl+P)",
                    span { class: "icon-preview" }
                }
            }
            button {
                class: sidebar_class,
                onclick: move |_| on_toggle_sidebar.call(()),
                title: "Notes List (Ctrl+B)",
                span { class: "icon-menu" }
            }
            button {
                class: focus_class,
                onclick: move |_| on_toggle_focus.call(()),
                title: "Focus Mode (Ctrl+Shift+F)",
                span { class: "icon-focus", "⊙" }
            }
            if has_note {
                div { class: "toolbar-divider" }
                button {
                    class: "btn-toolbar btn-delete",
                    onclick: move |_| on_delete.call(()),
                    title: "Delete Note",
                    span { class: "icon-delete" }
                }
            }
        }
    }
}
