use dioxus::prelude::*;

use crate::elements::render_children;

pub fn render_emphasis(node: &markdown::mdast::Emphasis) -> Result<Element, Element> {
    let children = render_children(&node.children)?;

    Ok(rsx! {
        Emphasis {  {children.iter()} }
    })
}

#[component]
pub fn Emphasis(children: Element) -> Element {
    rsx! {
        em { {children.iter()} }
    }
}
