use std::fmt::Display;

use dioxus::prelude::*;
use dx_primitives::{
    either_selector::{EitherLabel, EitherSelector},
    input::{ColorInput, XyInput},
    ribbon_selector::{RibbonSelector, SelectEnum},
};
use either::Either;
use strum::{EnumIter, EnumProperty};

use crate::rand_color;

#[component]
pub fn Primitives() -> Element {
    #[derive(Clone, Copy, PartialEq, Eq)]
    struct Foo;
    impl EitherLabel for Foo {
        fn label() -> impl Display {
            "Foo"
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq)]
    struct Bar;
    impl EitherLabel for Bar {
        fn label() -> impl Display {
            "Bar"
        }
    }

    let mut ab: Signal<Either<Foo, Bar>> = use_signal(|| Either::Left(Foo));
    let on_select_a = use_callback(move |_| ab.set(Either::Left(Foo)));
    let on_select_b = use_callback(move |_| ab.set(Either::Right(Bar)));

    let mut ribbon_selected = use_signal(|| None);
    let select_ribbon = use_callback(move |rs| ribbon_selected.set(Some(rs)));

    let mut xy = use_signal(|| [-0.5, 0.5]);
    let onchange_xy = use_callback(move |new_xy| {
        xy.set(new_xy);
        info!(?new_xy, "updated xy");
    });

    rsx! {
        div { class: "grid grid-cols-4 gap-2 p-2",
            div { class: "border rounded",
                EitherSelector { select: ab(), on_select_a, on_select_b }
            }
            div { class: "border rounded",
                ColorInput { color: rand_color() }
            }
            div { class: "border rounded",
                RibbonSelector::<RibbonSelect> {
                    selected: ribbon_selected(),
                    on_select: select_ribbon
                }
            }
            div { class: "border rounded",
                XyInput { xy, onchange: onchange_xy }
            }
        }
    }
}

#[derive(Debug, EnumIter, EnumProperty, PartialEq, Eq, Clone, Copy)]
enum RibbonSelect {
    #[strum(props(label = "cyan"))]
    Cyan,
    #[strum(props(label = "yellow"))]
    Yellow,
    #[strum(props(label = "magenta"))]
    Magenta,
}

impl SelectEnum for RibbonSelect {
    fn label(&self) -> &str {
        self.get_str("label").unwrap_or("?")
    }
}
