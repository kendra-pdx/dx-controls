use crate::*;
use bevy_color::Color;
use dioxus::prelude::*;

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
