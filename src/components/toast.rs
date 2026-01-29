use dioxus::prelude::*;

#[component]
pub fn Toast(
    message: String,
    on_undo: EventHandler<()>,
    on_dismiss: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "toast",
            span { class: "toast-message", "{message}" }
            button {
                class: "toast-btn toast-undo",
                onclick: move |_| on_undo.call(()),
                "Undo"
            }
            button {
                class: "toast-btn toast-dismiss",
                onclick: move |_| on_dismiss.call(()),
                "×"
            }
        }
    }
}
