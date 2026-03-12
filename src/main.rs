use bevy_color::Color;
use dioxus::prelude::*;
use rand::RngExt;

use dx_gradient::{Stop, Stops};

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let floats = use_store(|| {
        vec![
            Stop::new(0.0, 0.2),
            Stop::new(0.3, 1.0),
            Stop::new(1.0, 0.3),
        ]
    });

    let colors = use_store(|| {
        vec![
            Stop::new(0.0, rand_color()),
            Stop::new(0.7, rand_color()),
            Stop::new(1.0, rand_color()),
        ]
    });

    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "w-full flex-col m-4 border rounded",
            Stops { stops: floats }
            Stops { stops: colors }
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
