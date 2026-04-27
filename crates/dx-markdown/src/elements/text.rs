use dioxus::prelude::*;

pub fn render_text(node: &markdown::mdast::Text) -> Result<Element, Element> {
    Ok(rsx! {
        Text { {node.value.clone()} }
    })
}

#[component]
pub fn Text(children: Element) -> Element {
    rsx! {
        {children}
    }
}
