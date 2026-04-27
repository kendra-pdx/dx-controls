#![allow(dead_code)]
use dioxus::prelude::*;
use strum::EnumProperty;

#[component]
pub fn FontFace(
    url: Asset,
    font_family: &'static str,

    #[props(default)] format: FontFormat,
    #[props(default)] font_style: FontStyle,
    #[props(default = "100 900")] font_weight: &'static str,
) -> Element {
    let format = format.format();
    let font_style = font_style.style();
    rsx! {
        document::Style {{
            format!(
                "@font-face {{
                    font-family: '{font_family}';
                    src: url('{url}') format('{format}');
                    font-weight: {font_weight};
                    font-style: {font_style};
                    font-display: swap;
                }}",
            )
        }}
    }
}

#[derive(EnumProperty, Default, PartialEq, Eq, Clone, Copy)]
pub enum FontStyle {
    #[default]
    #[strum(props(style = "normal"))]
    Normal,
    #[strum(props(style = "italic"))]
    Italic,
}

#[derive(EnumProperty, Default, PartialEq, Eq, Clone, Copy)]
pub enum FontFormat {
    #[strum(props(format = "truetype"))]
    TrueType,

    #[default]
    #[strum(props(format = "woff2"))]
    Woff2,
}

impl FontStyle {
    pub fn style(&self) -> &str {
        self.get_str("style")
            .expect("FontStyle variants must have a 'style' property")
    }
}

impl FontFormat {
    pub fn format(&self) -> &str {
        self.get_str("format")
            .expect("FontFormat variants must have a 'format' property")
    }
}
