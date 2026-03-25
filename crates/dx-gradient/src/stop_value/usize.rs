use crate::*;
use bevy_color::Color;
use dioxus::prelude::*;

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
