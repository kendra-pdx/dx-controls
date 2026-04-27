use dioxus::prelude::*;

pub fn render_code(node: &markdown::mdast::Code) -> Result<Element, Element> {
    Ok(rsx! {
        Code { {node.value.clone()} }
    })
}

#[component]
pub fn Code(children: Element) -> Element {
    rsx! {
        pre { class: "md-code", {children} }
    }
}
