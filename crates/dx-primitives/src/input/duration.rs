use std::time::Duration;

use dioxus::prelude::*;

#[component]
fn DurationInput(value: ReadSignal<Duration>, onchange: Callback<Duration>) -> Element {
    rsx! {
        div { class: "flex flex-row gap-1"
        }
    }
}
