use bevy_color::Color;
use dioxus::prelude::*;

mod color;
mod duration;
mod f32;
mod usize;

pub trait StopValue: Copy {
    fn new(left: Self, right: Self) -> Self;
    fn edit(&self, on_change: Callback<Self>) -> Element;
    fn as_color(&self, all: impl IntoIterator<Item = Self>) -> Color;
}
