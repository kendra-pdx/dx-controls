use std::time::Duration;

use bevy_color::Color;
use dioxus::prelude::*;
use rand::RngExt;

use dx_gradient::{Stop, Stops};
#[component]
pub fn Gradients() -> Element {
    let mut rng = rand::rng();
    let floats = use_store(|| {
        vec![
            Stop::new(0.0, 0.2),
            Stop::new(rng.random_range(0.25..0.75), 1.0),
            Stop::new(1.0, 0.3),
        ]
    });

    let durations = use_store(|| {
        vec![
            Stop::new(0.0, Duration::from_millis(0)),
            Stop::new(1.0, Duration::from_millis(54321)),
        ]
    });

    let usizes = use_store(|| {
        vec![
            Stop::new(0.0, 2_usize),
            Stop::new(rng.random_range(0.25..0.75), 5),
            Stop::new(1.0, 2),
        ]
    });

    let colors = use_store(|| {
        vec![
            Stop::new(0.0, rand_color()),
            Stop::new(rng.random_range(0.25..0.75), rand_color()),
            Stop::new(1.0, rand_color()),
        ]
    });

    rsx! {
        div { class: "flex flex-col w-full h-full gap-2 p-2 border rounded",
            div { class: "flex flex-col",
                label { "floats" }
                Stops { stops: floats }
            }

            div { class: "flex flex-col",
                label { "durations" }
                Stops { stops: durations }
            }

            div { class: "flex flex-col",
                label { "colors" }
                Stops { stops: colors }
            }

            div { class: "flex flex-col",
                label { "usizes" }
                Stops { stops: usizes }
            }
        }
    }
}

fn rand_color() -> Color {
    let mut rng = rand::rng();
    let h: f32 = rng.random::<f32>() * 360.0;
    let s = rng.random_range(0.5..0.8);
    let v = rng.random_range(0.5..0.8);
    Color::hsv(h, s, v)
}
