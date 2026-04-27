use dioxus::prelude::*;

use crate::elements::render_children;

pub fn render_link(node: &markdown::mdast::Link) -> Result<Element, Element> {
    let url = node.url.clone();
    let title = node.title.clone();
    let children = render_children(&node.children)?;

    Ok(rsx! {
        Link { url, title, {children.iter()} }
    })
}

#[component]
pub fn Link(url: String, title: Option<String>, children: Element) -> Element {
    rsx! {
        a { href: url, alt: title, {children.iter()} }
    }
}
