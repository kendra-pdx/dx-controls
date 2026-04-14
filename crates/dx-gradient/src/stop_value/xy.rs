use std::{fmt::Display, str::FromStr};

use bevy_color::Color;
use dioxus::prelude::*;
use dx_primitives::input::{XY, XyInput};
use num_traits::Num;
use spacial::prelude::TwizzleXY;

use crate::StopValue;

impl<T> StopValue for XY<T>
where
    T: PartialEq + Copy + Num + Display + FromStr + Ord + 'static,
{
    fn new(left: Self, right: Self) -> Self {
        let x = left.x().min(right.x());
        let y = left.y().min(right.y());
        [x, y]
    }

    fn edit(&self, on_change: dioxus::prelude::Callback<Self>) -> dioxus::prelude::Element {
        rsx! {
            XyInput { xy: *self, onchange: on_change }
        }
    }

    fn as_color(&self, _all: impl IntoIterator<Item = Self>) -> bevy_color::Color {
        // XY doesn't represent well as a color. maybe LAB?
        Color::srgb(0.5, 0.5, 0.5)
    }
}
