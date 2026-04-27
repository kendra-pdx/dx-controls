use dioxus::prelude::*;

use crate::elements::*;

pub fn render_paragraph(node: &markdown::mdast::Paragraph) -> Result<Element, Element> {
    let children = render_children(&node.children)?;

    Ok(rsx! {
        Paragraph { {children.iter()} }
    })
}

#[component]
pub fn Paragraph(children: Element) -> Element {
    rsx! {
        p { {children.iter()} }
    }
}
