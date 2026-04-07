use bevy_color::{Color, Mix};
use dioxus::prelude::*;
use dx_primitives::input::ColorInput;

use crate::StopValue;

impl StopValue for Color {
    fn new(left: Color, right: Color) -> Self {
        left.mix(&right, 0.5)
    }

    fn edit(&self, on_change: Callback<Self>) -> Element {
        rsx! {
            ColorInput { color: *self, onchange: on_change }
        }
    }

    fn as_color(&self, _all: impl IntoIterator<Item = Self>) -> Color {
        *self
    }
}
