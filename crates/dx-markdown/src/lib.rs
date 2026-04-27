mod elements;

use dioxus::{asset_resolver, prelude::*};
use markdown::ParseOptions;

use crate::elements::{md_element_error, MdElementError, NodeExt};

#[component]
pub fn Markdown(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    md: Asset,
) -> Element {
    let md = use_resource(move || async move { load_md_asset(&md).await.unwrap_or_else(|e| e) })
        .suspend()?;

    rsx! {
        div { ..attributes,
            {md.read().clone()}
        }
    }
}

async fn load_md_asset(asset: &Asset) -> Result<Element, Element> {
    let parse_options = ParseOptions::default();
    let bytes = asset_resolver::read_asset_bytes(asset).await.map_err(|e| {
        rsx! {
            MdElementError { message: format!("{e:?}") }
        }
    })?;

    let text = String::from_utf8_lossy(&bytes);
    let ast = markdown::to_mdast(&text, &parse_options).map_err(md_element_error)?;
    (&ast).try_into_element()
}
