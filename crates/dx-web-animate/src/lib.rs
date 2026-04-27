use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::*;

use derive_new::new;
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, Copy, new)]
pub struct AnimationTime {
    #[new(value = "Duration::ZERO")]
    pub delta_t: Duration,

    #[new(value = "Duration::ZERO")]
    prev_t: Duration,
}

impl AnimationTime {
    fn update_from_duration_ms(&mut self, total_ms: f64) {
        let new_t = Duration::from_secs_f64(total_ms / 1000.0);
        // On the first frame, prev_t is ZERO so (new_t - 0) would be the
        // entire time since page load — potentially seconds.  Emit a
        // zero-length delta instead so consumers don't get a giant kick.
        self.delta_t = if self.prev_t.is_zero() {
            Duration::ZERO
        } else {
            new_t - self.prev_t
        };
        self.prev_t = new_t;
    }

    pub fn total_elapsed(&self) -> Duration {
        self.prev_t + self.delta_t
    }
}

/// Runs `callback` on every animation frame using the browser's
/// [`requestAnimationFrame`](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame)
/// API.
///
/// The callback receives the
/// [`DOMHighResTimeStamp`](https://developer.mozilla.org/en-US/docs/Web/API/DOMHighResTimeStamp)
/// (milliseconds elapsed since the page was first loaded) as its argument.
///
/// The loop starts once on the first render and stops automatically when the
/// calling component is unmounted.
///
/// # Example
///
/// ```rust,ignore
/// #[component]
/// fn Animated() -> Element {
///     let mut angle = use_signal(|| 0.0_f64);
///     use_request_animation_frame(move |_ts| {
///         angle += 0.5;
///     });
///     rsx! { div { style: "transform: rotate({angle()}deg);", "spinning" } }
/// }
/// ```
pub fn use_request_animation_frame(callback: impl FnMut(AnimationTime) -> bool + 'static) {
    // Rc<Cell<bool>> is Clone (via Rc::clone, sharing the Cell), so use_hook
    // can store a clone while returning the original — both point to the same
    // Cell, and dropping either just decrements the Rc refcount.
    let animation_time = AnimationTime::new();
    let active = use_hook(|| {
        let active = Rc::new(Cell::new(true));
        schedule_raf(
            active.clone(),
            animation_time,
            true,
            Rc::new(RefCell::new(callback)),
        );
        active
    });

    use_drop(move || active.set(false));
}

// Each call registers exactly one Closure::once with the browser, then forgets
// it. Rust drops its bookkeeping; JS holds the only reference and GCs the
// closure after firing it. This avoids the self-referential Closure that would
// otherwise require leaking a Closure<dyn FnMut>.
//
// The callback is wrapped in Rc<RefCell<...>> so that FnMut can be invoked
// through a shared reference — necessary because the Closure itself is FnOnce.
fn schedule_raf(
    active: Rc<Cell<bool>>,
    mut animation_time: AnimationTime,
    first_frame: bool,
    callback: Rc<RefCell<dyn FnMut(AnimationTime) -> bool>>,
) {
    if !active.get() {
        return;
    }

    let win = web_sys::window().expect("no global `window` object");
    let active_clone = active.clone();
    let callback_clone = callback.clone();

    let closure = Closure::once(move |timestamp: f64| {
        animation_time.update_from_duration_ms(timestamp);
        if active_clone.get() {
            let stay_active = if !first_frame {
                (callback_clone.borrow_mut())(animation_time)
            } else {
                true
            };
            if stay_active {
                schedule_raf(active_clone, animation_time, false, callback_clone);
            }
        }
    });

    win.request_animation_frame(closure.as_ref().unchecked_ref())
        .expect("failed to schedule animation frame");

    closure.forget();
}
