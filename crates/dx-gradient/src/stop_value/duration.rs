use crate::StopValue;
use bevy_color::*;
use dioxus::prelude::*;
use std::time::Duration;

impl StopValue for Duration {
    fn new(left: Duration, right: Duration) -> Self {
        let left_ms = left.as_secs_f32();
        let right_ms = right.as_secs_f32();
        let mid = (left_ms + right_ms) * 0.5;
        Duration::from_secs_f32(mid)
    }

    fn edit(&self, on_change: Callback<Duration>) -> Element {
        let onchange = move |e: Event<FormData>| {
            let v: f32 = e.value().parse().expect("could not parse value as f32");
            let duration = Duration::from_secs_f32(v);
            on_change(duration);
        };

        let preview_value = format!("{self:?}",);
        let preview_class = if self.as_secs_f32() < 0.5 {
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
                        value: self.as_secs_f32().to_string(),
                        onchange,
                    }
                }
            }
        }
    }

    fn as_color(&self, all: impl IntoIterator<Item = Self>) -> Color {
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        all.into_iter().map(|d| d.as_secs_f32()).for_each(|v| {
            min = min.min(v);
            max = max.max(v);
        });
        let range = max - min;
        let t = (self.as_secs_f32() - min) / range;

        Color::srgb(t, t, t)
    }
}
