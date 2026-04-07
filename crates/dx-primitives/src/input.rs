mod color;
mod duration;

use dioxus::prelude::*;

pub use color::*;
pub use duration::*;

#[component]
pub fn FormField(label: ReadSignal<String>, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col",
            label { { label() }}
            { children }
        }
    }
}
