use std::{fmt::Debug, ops::Range};

use bevy_color::Color;
use derive_new::new;
use dioxus::prelude::*;
use glam::Vec2;

pub trait StopValue: Clone {
    fn edit(&self, on_change: Callback<Self>) -> Element;
    fn as_color(&self) -> Color;
}

#[derive(new, Store)]
pub struct Stop<V> {
    at: f32,
    value: V,
}

#[store]
impl<Lens, V: StopValue + 'static> Store<Stop<V>, Lens> {
    fn can_drag(&self) -> bool {
        let at = self.at()();
        at > 0.0 && at < 1.0
    }
}

#[component]
pub fn Stops<V: StopValue + Debug + 'static>(mut stops: Store<Vec<Stop<V>>>) -> Element {
    let mut size = use_signal(|| Vec2::new(200., 32.));

    let width = use_memo(move || size().x);
    let h_width = use_memo(move || *width.read() / 2.0);
    let height = use_memo(move || size().y);
    let h_height = use_memo(move || *height.read() / 2.0);

    let on_resize = use_callback(move |e: Event<ResizeData>| {
        if let Ok(size_data) = e.get_content_box_size() {
            info!("size: {e:?}");
            let [w, _] = size_data.to_f32().to_array();
            *size.write() = Vec2::new(w, height())
        }
    });
    let view_box =
        use_memo(move || format!("{} {} {} {}", -h_width(), -h_height(), width(), height()));

    let x_range = use_memo(move || -h_width()..h_width());

    let mut dragging = use_signal(move || None::<usize>);
    let mouse_move = use_callback(move |e: Event<MouseData>| {
        if let Some(stop_ix) = dragging() {
            let x_pos = e.data.element_coordinates().to_f32().x;
            let at = x_pos / width();
            // debug!(?e, stop_ix, x_pos, at, "dragging");
            if let Some(mut stop) = stops.get_mut(stop_ix) {
                stop.at = at;
            }
        }
    });

    let start_dragging_for = move |ix| {
        move |_| {
            if stops.get(ix).is_some_and(|s| s.can_drag()) {
                dragging.set(Some(ix));
            }
        }
    };

    let stop_dragging = move |_| dragging.set(None);

    let mut selected = use_signal(move || None::<usize>);
    let select_for = move |ix| {
        move |_| {
            debug!(ix, "selected stop");
            selected.set(Some(ix));
        }
    };

    let editor: Option<Element> = selected().and_then(|ix| stops.get(ix)).map(|stop| {
        let on_change = use_callback(move |v: V| {
            stop.value().set(v);
        });
        stop.value()().edit(on_change)
    });

    let bg_style = use_memo(move || {
        let segments = stops
            .iter()
            .map(|s| {
                let at = s.at() * 100.0;
                let color = s.value()().as_color().to_srgba().to_hex();
                format!("{color} {at}%")
            })
            .collect::<Vec<_>>()
            .join(", ");
        let gradient = format!("linear-gradient(to right, {segments})");
        format!("background: {gradient}")
    });

    rsx! {
        div { class: "p-4 flex flex-col gap-2",
            svg { view_box, class: "w-full border rounded border-gray-400",  style: "{bg_style}", preserve_aspect_ratio: "xMidYMid slice",
                onresize: on_resize, onmousemove: mouse_move, onmouseup: stop_dragging,
                line { class: "stroke-gray-300", x1: -h_width(), x2: h_width() }
                for (i, stop) in stops.iter().enumerate() {
                    StopHandle { at: stop.at(), x_range,
                        on_dragging: start_dragging_for(i),
                        on_select: select_for(i)
                    }
                }
            }
            {editor}
       }
    }
}

#[component]
fn StopHandle(
    at: Store<f32>,
    x_range: ReadSignal<Range<f32>>,
    on_dragging: Callback,
    on_select: Callback,
) -> Element {
    let cx = (at() * (x_range().end - x_range().start)) + x_range().start;
    rsx! {
        circle { fill: "black", r: 6, cx, cy: 0,
            onmousedown: move |_| on_dragging(()),
            onclick: move |_| on_select(())
        }
    }
}

impl StopValue for f32 {
    fn edit(&self, on_change: Callback<f32>) -> Element {
        let v = format!("{self}");
        let onchange = move |e: Event<FormData>| {
            let v: f32 = e.value().parse().expect("could not parse value as f32");
            on_change(v);
        };
        rsx! {
            div { class: "flex flex-row gap-2 w-full",
                div { class: "flex-none", {v.clone()} }
                div { class: "grow",
                    input { type: "range", min: "0", max: "1", step: "0.01", class: "mx-4 w-full",
                        value: v, onchange
                    }
                }
            }
        }
    }

    fn as_color(&self) -> Color {
        Color::srgb(*self, *self, *self)
    }
}
