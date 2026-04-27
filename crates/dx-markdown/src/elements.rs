mod code;
mod emphasis;
mod heading;
mod inline_code;
mod link;
mod list;
mod paragraph;
mod root;
mod strong;
mod text;

use dioxus::prelude::*;
use markdown::mdast;

use crate::elements::paragraph::render_paragraph;

pub fn md_element_error(message: impl ToString) -> Element {
    rsx! {
        MdElementError { message: message.to_string() }
    }
}

#[component]
pub fn MdElementError(message: String) -> Element {
    rsx! {
        div { {message} }
    }
}

pub trait NodeExt<'n> {
    fn try_into_element(&'n self) -> Result<Element, Element>;
}

impl<'n> NodeExt<'n> for &'n markdown::mdast::Node {
    fn try_into_element(&'n self) -> Result<Element, Element> {
        let element = match self {
            markdown::mdast::Node::Root(root) => root::render_root(root)?,
            // markdown::mdast::Node::Blockquote(blockquote) => todo!(),
            // markdown::mdast::Node::FootnoteDefinition(footnote_definition) => todo!(),
            // markdown::mdast::Node::MdxJsxFlowElement(mdx_jsx_flow_element) => todo!(),
            markdown::mdast::Node::List(list) => list::render_list(list)?,
            // markdown::mdast::Node::MdxjsEsm(mdxjs_esm) => todo!(),
            // markdown::mdast::Node::Toml(toml) => todo!(),
            // markdown::mdast::Node::Yaml(yaml) => todo!(),
            // markdown::mdast::Node::Break(_) => todo!(),
            markdown::mdast::Node::InlineCode(inline_code) => {
                inline_code::render_inline_code(inline_code)?
            }
            // markdown::mdast::Node::InlineMath(inline_math) => todo!(),
            // markdown::mdast::Node::Delete(delete) => todo!(),
            markdown::mdast::Node::Emphasis(emphasis) => emphasis::render_emphasis(emphasis)?,
            // markdown::mdast::Node::MdxTextExpression(mdx_text_expression) => todo!(),
            // markdown::mdast::Node::FootnoteReference(footnote_reference) => todo!(),
            // markdown::mdast::Node::Html(html) => todo!(),
            // markdown::mdast::Node::Image(image) => todo!(),
            // markdown::mdast::Node::ImageReference(image_reference) => todo!(),
            // markdown::mdast::Node::MdxJsxTextElement(mdx_jsx_text_element) => todo!(),
            markdown::mdast::Node::Link(link) => link::render_link(link)?,
            // markdown::mdast::Node::LinkReference(link_reference) => todo!(),
            markdown::mdast::Node::Strong(strong) => strong::render_strong(strong)?,
            markdown::mdast::Node::Text(text) => text::render_text(text)?,
            markdown::mdast::Node::Code(code) => code::render_code(code)?,
            // markdown::mdast::Node::Math(math) => todo!(),
            // markdown::mdast::Node::MdxFlowExpression(mdx_flow_expression) => todo!(),
            markdown::mdast::Node::Heading(heading) => heading::render_heading(heading)?,
            // markdown::mdast::Node::Table(table) => todo!(),
            // markdown::mdast::Node::ThematicBreak(thematic_break) => todo!(),
            // markdown::mdast::Node::TableRow(table_row) => todo!(),
            // markdown::mdast::Node::TableCell(table_cell) => todo!(),
            markdown::mdast::Node::ListItem(list_item) => list::render_list_item(list_item)?,
            // markdown::mdast::Node::Definition(definition) => todo!(),
            markdown::mdast::Node::Paragraph(paragraph) => render_paragraph(paragraph)?,
            n => {
                let message = format!("node ast not implemented: {:?}", n);
                return Err(rsx! {
                    MdElementError { message }
                });
            }
        };
        Ok(element)
    }
}

fn render_children(children: &[mdast::Node]) -> Result<Vec<Element>, Element> {
    let children = children
        .iter()
        .map(|child| child.try_into_element())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(children)
}
