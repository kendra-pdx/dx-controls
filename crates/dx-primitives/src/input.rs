mod color;
mod duration;
mod xy;

use dioxus::prelude::*;

pub use color::*;
pub use duration::*;
pub use xy::*;

#[component]
pub fn FormField(label: ReadSignal<String>, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col",
            label { { label() }}
            { children }
        }
    }
}
