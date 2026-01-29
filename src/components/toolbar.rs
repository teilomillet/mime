use dioxus::prelude::*;

#[component]
pub fn Toolbar(
    is_sidebar_visible: bool,
    is_preview_visible: bool,
    has_note: bool,
    on_toggle_sidebar: EventHandler<()>,
    on_toggle_preview: EventHandler<()>,
    on_delete: EventHandler<()>,
) -> Element {
    let sidebar_class = if is_sidebar_visible { "btn-toolbar active" } else { "btn-toolbar" };
    let preview_class = if is_preview_visible { "btn-toolbar active" } else { "btn-toolbar" };

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
