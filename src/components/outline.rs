use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HeadingInfo {
    pub level: u8,
    pub text: String,
    pub line: usize,
}

pub fn extract_headings(content: &str) -> Vec<HeadingInfo> {
    content
        .lines()
        .enumerate()
        .filter_map(|(line_num, line)| {
            let trimmed = line.trim_start();
            if trimmed.starts_with('#') {
                let level = trimmed.chars().take_while(|c| *c == '#').count() as u8;
                if level <= 6 {
                    let text = trimmed[level as usize..].trim().to_string();
                    if !text.is_empty() {
                        return Some(HeadingInfo {
                            level,
                            text,
                            line: line_num,
                        });
                    }
                }
            }
            None
        })
        .collect()
}

#[component]
pub fn Outline(content: String, on_jump: EventHandler<usize>) -> Element {
    let headings = extract_headings(&content);

    rsx! {
        aside { class: "outline",
            div { class: "outline-track",
                for heading in headings {
                    div {
                        class: "outline-item level-{heading.level}",
                        title: "{heading.text}",
                        onclick: {
                            let line = heading.line;
                            move |_| on_jump.call(line)
                        },
                        span { class: "outline-indicator" }
                        span { class: "outline-text", "{heading.text}" }
                    }
                }
            }
        }
    }
}
