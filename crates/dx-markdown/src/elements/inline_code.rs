use dioxus::prelude::*;

pub fn render_inline_code(node: &markdown::mdast::InlineCode) -> Result<Element, Element> {
    Ok(rsx! {
        InlineCode { {node.value.clone()} }
    })
}

#[component]
pub fn InlineCode(children: Element) -> Element {
    rsx! {
        code { {children} }
    }
}
