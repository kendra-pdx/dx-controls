mod gradient;
mod primitives;

use dioxus::prelude::*;

const BUNDLE_CSS: Asset = asset!("/assets/bundle.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BUNDLE_CSS }
        div { class: "flex flex-col h-full w-full overflow-none",
            primitives::Primitives {  }
            gradient::Gradients {}
        }
    }
}
