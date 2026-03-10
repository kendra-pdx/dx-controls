use std::ops::Range;

use derive_new::new;
use dioxus::prelude::*;
use glam::Vec2;

#[derive(new, Store)]
struct Stop {
    at: f32,
}

#[store]
impl<Lens> Store<Stop, Lens> {
    fn can_drag(&self) -> bool {
        let at = self.at()();
        at > 0.0 && at < 1.0
    }
}

#[component]
pub fn Stops() -> Element {
    let mut size = use_signal(|| Vec2::new(200., 20.));

    let on_resize = use_callback(move |e: Event<ResizeData>| {
        if let Ok(size_data) = e.get_content_box_size() {
            info!("size: {e:?}");
            let [w, _] = size_data.to_f32().to_array();
            *size.write() = Vec2::new(w, 20.)
        }
    });

    let width = use_memo(move || size().x);
    let h_width = use_memo(move || *width.read() / 2.0);
    let height = use_memo(move || size().y);
    let h_height = use_memo(move || *height.read() / 2.0);

    let view_box =
        use_memo(move || format!("{} {} {} {}", -h_width(), -h_height(), width(), height()));

    let mut stops = use_store(|| vec![Stop::new(0.0), Stop::new(0.3), Stop::new(1.0)]);

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

    rsx! {
        div { class: "p-4",
            svg { view_box,
                class: "w-full border rounded border-gray-400", onresize: on_resize, preserve_aspect_ratio: "xMidYMid keep",
                onmousemove: mouse_move, onmouseup: stop_dragging,
                line { class: "stroke-gray-300", x1: -h_width(), x2: h_width() }
                for (i, stop) in stops.iter().enumerate() {
                    StopHandle { at: stop.at(), x_range, on_dragging: start_dragging_for(i)  }
                }
            }
       }
    }
}

#[component]
fn StopHandle(at: Store<f32>, x_range: ReadSignal<Range<f32>>, on_dragging: Callback) -> Element {
    let cx = (at() * (x_range().end - x_range().start)) + x_range().start;
    rsx! {
        circle { fill: "black", r: 6, cx, cy: 0,
            onmousedown: move |_| on_dragging(()),
        }
    }
}
