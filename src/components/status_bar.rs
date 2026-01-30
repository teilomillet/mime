use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum TimerState {
    Stopped,
    Running,
    Paused,
}

const PRESET_DURATIONS: &[(u64, &str)] = &[
    (5 * 60, "5m"),
    (15 * 60, "15m"),
    (25 * 60, "25m"),
    (45 * 60, "45m"),
];

#[component]
pub fn StatusBar(
    #[props(default)] content: Option<String>,
    #[props(default)] has_note: bool,
    #[props(default)] is_sidebar_visible: bool,
    #[props(default)] is_preview_visible: bool,
    #[props(default)] is_focus_mode: bool,
    #[props(default)] font_size: u8,
    on_toggle_sidebar: EventHandler<()>,
    on_toggle_preview: EventHandler<()>,
    on_toggle_focus: EventHandler<()>,
    on_delete: EventHandler<()>,
    on_font_size_change: EventHandler<u8>,
) -> Element {
    let (word_count, _reading_time) = content
        .as_ref()
        .map(|c| {
            let words = c.split_whitespace().count();
            let minutes = (words as f64 / 200.0).ceil() as usize;
            (words, minutes.max(1))
        })
        .unwrap_or((0, 0));

    let mut remaining_seconds = use_signal(|| 25u64 * 60);
    let mut initial_seconds = use_signal(|| 25u64 * 60);
    let mut timer_state = use_signal(|| TimerState::Stopped);
    let mut is_hovered = use_signal(|| false);
    let mut tick_count = use_signal(|| 0u64);

    let _timer = use_resource(move || async move {
        loop {
            #[cfg(target_arch = "wasm32")]
            gloo_timers::future::TimeoutFuture::new(1000).await;

            #[cfg(not(target_arch = "wasm32"))]
            async_std::task::sleep(std::time::Duration::from_secs(1)).await;

            if timer_state() == TimerState::Running && remaining_seconds() > 0 {
                remaining_seconds.set(remaining_seconds() - 1);
                tick_count.set(tick_count() + 1);
                if remaining_seconds() == 0 {
                    timer_state.set(TimerState::Stopped);
                }
            }
        }
    });

    let hours = remaining_seconds() / 3600;
    let minutes = (remaining_seconds() % 3600) / 60;
    let seconds = remaining_seconds() % 60;

    let time_display = if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    };

    let state = timer_state();
    let is_finished = remaining_seconds() == 0;

    let sidebar_class = if is_sidebar_visible { "btn-bar active" } else { "btn-bar" };
    let preview_class = if is_preview_visible { "btn-bar active" } else { "btn-bar" };
    let focus_class = if is_focus_mode { "btn-bar active" } else { "btn-bar" };

    rsx! {
        div {
            class: "status-bar",
            class: if is_hovered() { "status-bar-visible" } else { "" },
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),

            div { class: "status-bar-content",
                // Left: View controls
                div { class: "bar-section",
                    button {
                        class: sidebar_class,
                        onclick: move |_| on_toggle_sidebar.call(()),
                        title: "Notes (Ctrl+B)",
                        "☰"
                    }
                    if has_note {
                        button {
                            class: preview_class,
                            onclick: move |_| on_toggle_preview.call(()),
                            title: "Preview (Ctrl+P)",
                            "◫"
                        }
                    }
                    button {
                        class: focus_class,
                        onclick: move |_| on_toggle_focus.call(()),
                        title: "Focus (Ctrl+Shift+F)",
                        "⊙"
                    }
                }

                // Center: Word count & timer
                div { class: "bar-section bar-center",
                    if word_count > 0 {
                        span { class: "word-count", "{word_count} words" }
                    }

                    div { class: "timer-group",
                        for (duration, label) in PRESET_DURATIONS.iter() {
                            button {
                                class: "btn-preset",
                                class: if initial_seconds() == *duration { "active" } else { "" },
                                onclick: move |_| {
                                    initial_seconds.set(*duration);
                                    remaining_seconds.set(*duration);
                                    timer_state.set(TimerState::Stopped);
                                },
                                "{label}"
                            }
                        }
                        span {
                            class: "timer-time",
                            class: if is_finished { "timer-finished" } else { "" },
                            "{time_display}"
                        }
                        match state {
                            TimerState::Stopped => rsx! {
                                button {
                                    class: "btn-timer",
                                    onclick: move |_| {
                                        if remaining_seconds() == 0 {
                                            remaining_seconds.set(initial_seconds());
                                        }
                                        timer_state.set(TimerState::Running);
                                    },
                                    "▶"
                                }
                            },
                            TimerState::Running => rsx! {
                                button {
                                    class: "btn-timer",
                                    onclick: move |_| timer_state.set(TimerState::Paused),
                                    "⏸"
                                }
                            },
                            TimerState::Paused => rsx! {
                                button {
                                    class: "btn-timer",
                                    onclick: move |_| timer_state.set(TimerState::Running),
                                    "▶"
                                }
                            },
                        }
                    }
                }

                // Right: Font size & delete
                div { class: "bar-section",
                    if has_note {
                        button {
                            class: "btn-bar",
                            onclick: move |_| {
                                if font_size > 14 {
                                    on_font_size_change.call(font_size - 2);
                                }
                            },
                            title: "Smaller text",
                            "A−"
                        }
                        button {
                            class: "btn-bar",
                            onclick: move |_| {
                                if font_size < 32 {
                                    on_font_size_change.call(font_size + 2);
                                }
                            },
                            title: "Larger text",
                            "A+"
                        }
                        button {
                            class: "btn-bar btn-delete",
                            onclick: move |_| on_delete.call(()),
                            title: "Delete note",
                            "🗑"
                        }
                    }
                }
            }
        }
    }
}
