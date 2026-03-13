use std::fmt::Debug;

use bon::Builder;
use derive_new::new;
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, IconShape};
use strum::EnumProperty;

#[derive(new, Debug, Clone, Builder, PartialEq)]
pub struct Icon<I: IconShape + PartialEq + Clone + 'static> {
    class: Option<String>,
    icon: I,
}

#[derive(new, Debug, PartialEq, Clone, Builder)]
pub struct Text {
    class: Option<String>,
    text: String,
}

#[derive(Debug, Default, PartialEq, Clone, Copy, EnumProperty)]
pub enum StylePreset {
    #[strum(props(
        main_class = "cursor-pointer text-black border rounded border-black hover:bg-gray-100 hover:shadow-gray-500/50 hover:shadow-sm",
        icon_class = "",
        text_class = "text-sm hidden group-hover:flex"
    ))]
    #[default]
    Normal,

    #[strum(props(
        main_class = "cursor-pointer text-red-500 border rounded border-red-500 hover:bg-red-100 hover:shadow-red-500/50 hover:shadow-sm",
        icon_class = "",
        text_class = "text-sm hidden group-hover:flex"
    ))]
    Destructive,
}

#[derive(Debug, Props, PartialEq, Clone)]
pub struct IconButtonProps<I: IconShape + PartialEq + Clone + 'static> {
    #[props(default)]
    pub preset: StylePreset,

    pub class: Option<String>,

    pub icon: Option<Icon<I>>,

    pub text: Option<Text>,

    pub onclick: Option<EventHandler<MouseEvent>>,

    #[props(extends = GlobalAttributes)]
    globals: Vec<Attribute>,
}

#[component]
pub fn IconButton<I: IconShape + PartialEq + Clone + 'static>(
    props: IconButtonProps<I>,
) -> Element {
    let icon = |icon| render_icon(icon, props.preset);
    let text = |text| render_text(text, props.preset);

    let main_class = props
        .class
        .unwrap_or_else(|| props.preset.main_class().into());

    rsx! {
        div {
            class: "group flex flex-row gap-1 p-1 h-full place-items-center {main_class}",
            onclick: props.onclick.unwrap_or_default(),
            {props.icon.map(icon)}
            {props.text.map(text)}
        }
    }
}

fn render_icon<I: IconShape + PartialEq + Clone + 'static>(
    icon: Icon<I>,
    preset: StylePreset,
) -> Element {
    let class = icon
        .class
        .unwrap_or_else(|| preset.icon_class().to_string());

    rsx! {
        Icon { class,
            icon: icon.icon.clone(),
        }
    }
}

fn render_text(text: Text, preset: StylePreset) -> Element {
    let class = text
        .class
        .unwrap_or_else(|| preset.text_class().to_string());
    rsx! {
        div { class,
            {text.text}
        }
    }
}

impl StylePreset {
    fn main_class(&self) -> &str {
        self.get_str("main_class").unwrap_or_default()
    }

    fn text_class(&self) -> &str {
        self.get_str("text_class").unwrap_or_default()
    }

    fn icon_class(&self) -> &str {
        self.get_str("icon_class").unwrap_or_default()
    }
}

impl<I: IconShape + PartialEq + Clone + 'static> From<I> for Icon<I> {
    fn from(value: I) -> Self {
        Icon::new(None, value)
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Text::new(None, value)
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Text::new(None, value.to_string())
    }
}
