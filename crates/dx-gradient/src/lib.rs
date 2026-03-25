mod stop_value;

use std::fmt::Debug;
use std::ops::Range;

use derive_new::new;
use dioxus::prelude::*;
use dioxus_free_icons::icons::fi_icons::*;
use dx_primitives::button::IconButton;
use glam::Vec2;
use itertools::Itertools;

pub use stop_value::*;

#[derive(new, Store, Clone, Copy)]
pub struct Stop<V> {
    at: f32,
    value: V,
}

#[store]
impl<Lens, V: StopValue + 'static> Store<Stop<V>, Lens> {
    fn position_mutable(&self) -> bool {
        let at = self.at()();
        at > 0.0 && at < 1.0
    }
}

#[store]
impl<Lens, V: StopValue + 'static> Store<Vec<Stop<V>>, Lens> {
    fn create_at(&mut self, at: f32) {
        let position = self.iter().find_position(|si| at < si.at()());
        if let Some((ix, _)) = position {
            let ix = ix;
            info!(at, ix, "creating stop");
            assert!(ix > 0);
            assert!(ix < self.len());
            let left = self.read()[ix - 1];
            let right = self.read()[ix];

            let stop = Stop::new(at, V::new(left.value, right.value));
            self.insert(ix, stop);
        }
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
        if let Some(stop_ix) = dragging()
            && let Some(mut stop) = stops.get_mut(stop_ix)
        {
            let x_pos = e.data.element_coordinates().to_f32().x;
            let at = x_pos / width();
            stop.at = at;
        }
    });

    let start_dragging_for = move |ix| {
        move |_| {
            if stops.get(ix).is_some_and(|s| s.position_mutable()) {
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
    let deselect = move |_| selected.set(None);

    let on_change = use_callback(move |v| {
        selected().and_then(|ix| stops.get_mut(ix)).map(|mut stop| {
            stop.value = v;
        });
    });

    let bg_style = use_memo(move || {
        let segments = stops
            .iter()
            .map(|s| {
                let all_vals = stops.iter().map(|s| s.value()());

                let at = s.at() * 100.0;
                let color = s.value()().as_color(all_vals).to_srgba().to_hex();
                format!("{color} {at}%")
            })
            .join(", ");
        let gradient = format!("linear-gradient(to right, {segments})");
        format!("background: {gradient}")
    });

    let editor: Option<Element> = selected()
        .and_then(|ix| stops.get(ix))
        .map(|stop| stop.value()().edit(on_change));

    let selected_removable = use_memo(move || {
        if let Some(ix) = selected() {
            ix != 0 && ix != stops.len() - 1
        } else {
            false
        }
    });

    let remove_selected = use_callback(move |_| {
        if let Some(ix) = selected() {
            info!(ix, "removing stop at index");
            if ix != 0 && ix != stops.len() - 1 {
                // cannot delete the first or last index
                stops.remove(ix);
            }
            selected.set(None);
        }
    });

    let create_stop = use_callback(move |e: Event<MouseData>| {
        if dragging().is_none() {
            let x_pos = e.data.element_coordinates().to_f32().x;
            let at = x_pos / width();

            info!(at, "create a new stop at mouse x");
            stops.create_at(at);
        }
    });

    rsx! {
        div { class: "flex flex-col gap-2 p-4",
            svg {
                view_box,
                class: "w-full border rounded border-gray-400",
                style: "{bg_style}",
                preserve_aspect_ratio: "xMidYMid slice",
                onresize: on_resize,
                onmousemove: mouse_move,
                onmouseup: stop_dragging,
                onclick: create_stop,
                defs {
                    g { id: "handle",
                        rect { x: -8, y: -8, width: 16, height: 16, rx: 3, ry: 2,
                            class: "stroke-black/90 fill-black/50",
                            stroke_width: "1.5",
                        }
                        rect { x: -9, y: -9, width: 18, height: 18, rx: 3, ry: 2,
                            class: "stroke-white/90 fill-none",
                        }
                    }
                }
                line { class: "stroke-gray-300", x1: -h_width(), x2: h_width() }
                for (i, stop) in stops.iter().enumerate() {
                    StopHandle {
                        at: stop.at(),
                        x_range,
                        on_dragging: start_dragging_for(i),
                        on_select: select_for(i),
                    }
                }
            }
            {
                editor
                    .map(|editor| {
                        rsx! {
                            div { class: "flex flex-row items-center gap-2",
                                div { class: "flex flex-row gap-1",
                                    if selected_removable() {
                                        IconButton {
                                            icon: Some(FiTrash2.into()),
                                            text: Some("delete".into()),
                                            preset: dx_primitives::button::StylePreset::Destructive,
                                            onclick: remove_selected,
                                        }
                                    }
                                    IconButton {
                                        icon: Some(FiCheck.into()),
                                        onclick: deselect,
                                    }
                                }
                                {editor}
                            }
                        }
                    })
            }
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
    let x = (at() * (x_range().end - x_range().start)) + x_range().start;
    let select = move |e: Event<MouseData>| {
        e.stop_propagation();
        on_select(());
    };
    rsx! {
        use { href: "#handle", x,
            onmousedown: move |_| on_dragging(()),
            onclick: select,
        }
    }
}
