mod stops;

use dioxus::prelude::*;

use crate::stops::{Stop, Stops};

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let stops = use_store(|| {
        vec![
            Stop::new(0.0, 0.2),
            Stop::new(0.3, 1.0),
            Stop::new(1.0, 0.3),
        ]
    });

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "w-full flex-col m-4 border rounded",
            Stops { stops }
        }
    }
}
