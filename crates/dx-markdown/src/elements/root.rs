use dioxus::prelude::*;

use crate::elements::*;

pub fn render_root(root: &markdown::mdast::Root) -> Result<Element, Element> {
    let children = render_children(&root.children)?;

    Ok(rsx! {
        Root { {children.iter()} }
    })
}

#[component]
pub fn Root(children: Element) -> Element {
    rsx! {
        div { class: "markdown", {children} }
    }
}
