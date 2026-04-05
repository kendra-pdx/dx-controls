use std::fmt::Display;

use dioxus::prelude::*;
use dx_primitives::either_selector::{EitherLabel, EitherSelector};
use either::Either;

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

    rsx! {
        div { class: "grid grid-rows-4",
            div {
                EitherSelector { select: ab(), on_select_a, on_select_b }
            }
        }
    }
}
