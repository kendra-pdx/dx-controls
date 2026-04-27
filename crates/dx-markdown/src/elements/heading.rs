use dioxus::prelude::*;

use crate::elements::*;

pub fn render_heading(node: &markdown::mdast::Heading) -> Result<Element, Element> {
    let children = render_children(&node.children)?;

    Ok(rsx! {
        Heading { depth: node.depth, {children.iter()} }
    })
}

#[component]
pub fn Heading(children: Element, depth: u8) -> Element {
    match depth {
        1 => rsx! {
            h1 { {children} }
        },
        2 => rsx! {
            h2 { {children} }
        },
        3 => rsx! {
            h3 { {children} }
        },
        _ => rsx! {
            h4 { {children} }
        },
    }
}
