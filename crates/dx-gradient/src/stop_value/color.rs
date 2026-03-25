use bevy_color::{Color, Mix};
use dioxus::prelude::*;

use crate::StopValue;

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
