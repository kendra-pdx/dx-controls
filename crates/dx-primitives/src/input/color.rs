use bevy_color::{Color, Hsva};
use dioxus::prelude::*;
use either::Either;

use crate::either_selector::{EitherLabel, EitherSelector};

#[component]
pub fn ColorInput(
    color: Color,
    #[props(default)] onchange: Callback<Color>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut color = use_signal(move || color);
    let mut colorspace = use_signal(|| Either::<_, Rgb>::Left(Hsv));

    let mut color_hex = use_signal(move || color().to_srgba().to_hex());

    let on_color_change = use_callback(move |e: Event<FormData>| {
        color_hex.set(e.value().clone());
    });

    use_effect(move || {
        let new_color = parse_hex_string(&*color_hex.read()).expect("must be able to parse color");

        match *colorspace.read() {
            Either::Left(_) => color.set(Hsva::from(new_color.to_srgba()).into()),
            Either::Right(_) => color.set(new_color),
        }

        onchange(color())
    });

    let on_select_hsv = use_callback(move |_| colorspace.set(Either::Left(Hsv)));
    let on_select_rgb = use_callback(move |_| colorspace.set(Either::Right(Rgb)));

    let color_desc = use_memo(move || match &*color.read() {
        Color::Srgba(srgba) => srgba.to_hex(),
        Color::Hsva(hsva) => format!(
            "HSV({:.02}, {:.02}, {:.02})",
            hsva.hue, hsva.saturation, hsva.value
        ),
        other => format!("{other:?}"),
    });

    rsx! {
        div { class: "flex flex-row w-full", ..attributes,
            div { class: "flex flex-row gap-2 items-center",
                EitherSelector { select: colorspace(), on_select_a: on_select_hsv, on_select_b: on_select_rgb }
                input { class: "w-8 h-full", type: "color", value: color_hex, onchange: on_color_change }
            }
            div { class: "flex",
                span { class: "text-xs", {color_desc} }
            }
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hsv;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rgb;

impl EitherLabel for Hsv {
    fn label() -> impl std::fmt::Display {
        "HSV"
    }
}

impl EitherLabel for Rgb {
    fn label() -> impl std::fmt::Display {
        "RGB"
    }
}

#[cfg(test)]
mod tests {
    use crate::input::parse_hex_string;

    #[test]
    fn parse_rgb_hex() {
        let ok = ["#ABC", "#123", "#AABBCC", "#abc123"];
        ok.into_iter().for_each(|ok| {
            assert!(parse_hex_string(ok).is_some());
        });

        let bad = ["abc", "#xyz", "#abc1"];
        bad.into_iter()
            .for_each(|bad| assert!(parse_hex_string(bad).is_none()));
    }
}
