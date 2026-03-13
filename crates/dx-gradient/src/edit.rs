use std::usize;

use bevy_color::{Color, Mix};
use dioxus::prelude::*;

use crate::StopValue;

impl StopValue for f32 {
    fn new(left: f32, right: f32) -> Self {
        // average of the two
        (left + right) / 2.0
    }

    fn edit(&self, on_change: Callback<f32>) -> Element {
        let onchange = move |e: Event<FormData>| {
            let v: f32 = e.value().parse().expect("could not parse value as f32");
            on_change(v);
        };

        let preview_value = format!("{self:0.3}");
        let preview_class = if *self < 0.5 {
            "border-white text-white"
        } else {
            "border-black text-black"
        };

        let bg_style = {
            let color = self.as_color(None).to_srgba().to_hex();
            format!("background-color: {color}")
        };

        rsx! {
            div { class: "flex flex-row gap-2 h-full w-full place-items-center",
                div { class: "flex flex-none text-xs p-1 border rounded border-gray-500 {preview_class}",
                    style: "{bg_style}",
                    {preview_value}
                }
                div { class: "flex grow",
                    input {
                        r#type: "range",
                        min: "0",
                        max: "1",
                        step: "0.01",
                        class: "mx-4 w-full",
                        value: self.to_string(),
                        onchange,
                    }
                }
            }
        }
    }

    fn as_color(&self, _all: impl IntoIterator<Item = Self>) -> Color {
        Color::srgb(*self, *self, *self)
    }
}

impl StopValue for usize {
    fn new(left: usize, right: usize) -> Self {
        // average of the two
        (left + right) / 2
    }

    fn edit(&self, on_change: Callback<usize>) -> Element {
        let onchange = move |e: Event<FormData>| {
            let value = e.value().parse().unwrap();
            on_change(value);
        };

        rsx! {
            div { class: "flex flex-row gap-2 h-full w-full place-items-center",
                input { class: "border rounded border-black p-1 mx-4 w-full",
                    r#type: "number",
                    min: "0",
                    value: self.to_string(),
                    onchange,
                }
            }
        }
    }

    fn as_color(&self, all: impl IntoIterator<Item = Self>) -> Color {
        let mut min = usize::MAX;
        let mut max = usize::MIN;
        all.into_iter().for_each(|v| {
            min = min.min(v);
            max = max.max(v);
        });
        let range = max - min;
        let t = (*self - min) as f32 / range as f32;
        Color::srgb(t, t, t)
    }
}

impl StopValue for Color {
    fn new(left: Color, right: Color) -> Self {
        left.mix(&right, 0.5)
    }

    fn edit(&self, on_change: Callback<Self>) -> Element {
        let hex = self.to_srgba().to_hex();
        let onchange = move |e: Event<FormData>| {
            let hex = e.value();
            info!(hex, "color changed");
            if let Some(color) = parse_hex_string(&e.value()) {
                on_change(color);
            } else {
                warn!(hex, "could not parse color string");
            }
        };
        rsx! {
            div { class: "flex flex-row h-full gap-2 w-full items-center",
                input {
                    r#type: "color",
                    class: "mx-4 w-full h-full",
                    value: hex,
                    onchange
                }
            }
        }
    }

    fn as_color(&self, _all: impl IntoIterator<Item = Self>) -> Color {
        *self
    }
}

pub fn parse_hex_string(hex: &str) -> Option<Color> {
    let hex = hex.trim();

    // Must start with #
    if !hex.starts_with('#') {
        warn!(hex, "hex value must begin with '#'");
        return None;
    }

    let hex_digits = &hex[1..];

    // Support both 3-digit and 6-digit formats
    let (r, g, b) = match hex_digits.len() {
        3 => {
            // Short format: #RGB -> #RRGGBB
            let r = u8::from_str_radix(&hex_digits[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex_digits[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex_digits[2..3].repeat(2), 16).ok()?;
            (r, g, b)
        }
        6 => {
            // Standard format: #RRGGBB
            let r = u8::from_str_radix(&hex_digits[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex_digits[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex_digits[4..6], 16).ok()?;
            (r, g, b)
        }
        _ => return None,
    };

    Some(Color::srgb_u8(r, g, b))
}
