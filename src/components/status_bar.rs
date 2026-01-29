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
    (60 * 60, "1h"),
];

#[component]
pub fn StatusBar() -> Element {
    let mut remaining_seconds = use_signal(|| 25u64 * 60);
    let mut initial_seconds = use_signal(|| 25u64 * 60);
    let mut timer_state = use_signal(|| TimerState::Stopped);
    let mut is_hovered = use_signal(|| false);
    let mut tick_count = use_signal(|| 0u64);

    // Use use_future that restarts when timer_state changes
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

    rsx! {
        div {
            class: "status-bar",
            class: if is_hovered() { "status-bar-visible" } else { "" },
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),

            div { class: "status-bar-content",
                // Duration presets
                div { class: "timer-presets",
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
                }

                // Timer display
                div { class: "timer-display",
                    span { 
                        class: "timer-time",
                        class: if is_finished { "timer-finished" } else { "" },
                        "{time_display}" 
                    }
                }

                // Timer controls
                div { class: "timer-controls",
                    match state {
                        TimerState::Stopped => rsx! {
                            button {
                                class: "btn-timer",
                                title: "Start timer",
                                onclick: move |_| {
                                    if remaining_seconds() == 0 {
                                        remaining_seconds.set(initial_seconds());
                                    }
                                    timer_state.set(TimerState::Running);
                                },
                                span { class: "icon-play" }
                            }
                        },
                        TimerState::Running => rsx! {
                            button {
                                class: "btn-timer",
                                title: "Pause timer",
                                onclick: move |_| timer_state.set(TimerState::Paused),
                                span { class: "icon-pause" }
                            }
                            button {
                                class: "btn-timer btn-stop",
                                title: "Reset timer",
                                onclick: move |_| {
                                    timer_state.set(TimerState::Stopped);
                                    remaining_seconds.set(initial_seconds());
                                },
                                span { class: "icon-stop" }
                            }
                        },
                        TimerState::Paused => rsx! {
                            button {
                                class: "btn-timer",
                                title: "Resume timer",
                                onclick: move |_| timer_state.set(TimerState::Running),
                                span { class: "icon-play" }
                            }
                            button {
                                class: "btn-timer btn-stop",
                                title: "Reset timer",
                                onclick: move |_| {
                                    timer_state.set(TimerState::Stopped);
                                    remaining_seconds.set(initial_seconds());
                                },
                                span { class: "icon-stop" }
                            }
                        },
                    }
                }
            }
        }
    }
}
