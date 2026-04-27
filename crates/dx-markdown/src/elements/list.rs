use dioxus::prelude::*;
use markdown::mdast;

use crate::elements::render_children;

pub fn render_list(node: &markdown::mdast::List) -> Result<Element, Element> {
    let children = render_children(&node.children)?;
    Ok(rsx! {
        List { ordered: node.ordered, {children.iter()} }
    })
}

pub fn render_list_item(node: &mdast::ListItem) -> Result<Element, Element> {
    let children = render_children(&node.children)?;
    Ok(rsx! {
        ListItem { {children.iter()} }
    })
}

#[component]
pub fn List(children: Element, ordered: bool) -> Element {
    if ordered {
        rsx! {
            ol { {children} }
        }
    } else {
        rsx! {
            ul { {children} }
        }
    }
}

#[component]
pub fn ListItem(children: Element) -> Element {
    rsx! {
        li { {children} }
    }
}
