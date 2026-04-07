mod gradient;
mod primitives;

use bevy_color::Color;
use dioxus::prelude::*;
use rand::RngExt;

const BUNDLE_CSS: Asset = asset!("/assets/bundle.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BUNDLE_CSS }
        div { class: "flex flex-col h-full w-full overflow-none",
            primitives::Primitives {  }
            gradient::Gradients {}
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
