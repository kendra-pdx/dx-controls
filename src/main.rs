mod stops;

use dioxus::prelude::*;

use crate::stops::Stops;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "w-full flex-col m-4 border rounded", Stops {
        } }
    }
}
