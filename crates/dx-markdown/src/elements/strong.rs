use dioxus::prelude::*;

use crate::elements::render_children;

pub fn render_strong(node: &markdown::mdast::Strong) -> Result<Element, Element> {
    let children = render_children(&node.children)?;

    Ok(rsx! {
        Strong {  {children.iter()} }
    })
}

#[component]
pub fn Strong(children: Element) -> Element {
    rsx! {
        strong { {children.iter()} }
    }
}
