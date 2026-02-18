mod components;
mod markdown;
mod state;
mod storage;

use components::App;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(|| {
        rsx! {
            document::Title { "Mime" }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            App {}
        }
    });
}
