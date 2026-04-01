use crate::StopValue;
use bevy_color::*;
use dioxus::prelude::*;
use dx_primitives::input::DurationInput;
use std::time::Duration;

impl StopValue for Duration {
    fn new(left: Duration, right: Duration) -> Self {
        let left_ms = left.as_secs_f32();
        let right_ms = right.as_secs_f32();
        let mid = (left_ms + right_ms) * 0.5;
        Duration::from_secs_f32(mid)
    }

    fn edit(&self, on_change: Callback<Duration>) -> Element {
        let value = *self;
        let duration_onchange = move |d: Duration| on_change.call(d);

        rsx! {
            div {class: "flex flex-row gap-2 h-full w-full place-items-center",
                DurationInput {
                    value,
                    onchange: duration_onchange
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
