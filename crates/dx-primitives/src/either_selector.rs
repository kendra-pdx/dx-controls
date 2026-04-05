use std::fmt::Display;

use dioxus::prelude::*;
use either::Either;
use strum::EnumProperty;

pub trait EitherLabel {
    fn label() -> impl Display;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EitherSelectorLayout {
    #[default]
    Horizontal,
}

#[component]
pub fn EitherSelector<A, B>(
    select: Either<A, B>,
    #[props(default)] on_select_a: Callback<()>,
    #[props(default)] on_select_b: Callback<()>,
    #[props(default)] layout: EitherSelectorLayout,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element
where
    A: 'static + EitherLabel + PartialEq + Clone,
    B: 'static + EitherLabel + PartialEq + Clone,
{
    match layout {
        EitherSelectorLayout::Horizontal => {
            render_horizontal(select, on_select_a, on_select_b, attributes)
        }
    }
}

fn render_horizontal<A, B>(
    select: Either<A, B>,
    on_select_a: Callback<()>,
    on_select_b: Callback<()>,
    attributes: Vec<Attribute>,
) -> Element
where
    A: 'static + EitherLabel,
    B: 'static + EitherLabel,
{
    let label_a = A::label().to_string();
    let label_b = B::label().to_string();

    let class_a = Style::which(&select, Selector::IsLeft).class();
    let class_b = Style::which(&select, Selector::IsRight).class();

    rsx! {
        div { class: "flex flex-row divide-x", ..attributes,
            div { class: "{class_a} border rounded-l-md",
                onclick: move |_| on_select_a.call(()),
                {label_a}
            }
            div { class: "{class_b} border rounded-r-md",
                onclick: move |_| on_select_b(()),
                {label_b}
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, EnumProperty)]
enum Style {
    #[strum(props(class = r#"
        flex px-2
        font-bold inset-shadow-gray-500 inset-shadow-sm

        cursor-pointer transition-colors duration-250
        bg-dx-surface1 text-dx-active

        hover:bg-dx-surface-hover hover:text-dx-hover
        hover:shadow-gray-500/50 hover:shadow-sm
        "#))]
    Selected,

    #[strum(props(class = r#"
        flex px-2

        cursor-pointer transition-colors duration-250

        bg-dx-surface1 text-dx-on-surface1

        hover:bg-dx-surface-hover hover:text-dx-hover
        hover:shadow-gray-500/50 hover:shadow-sm
        "#))]
    Unselected,
}

enum Selector {
    IsLeft,
    IsRight,
}

impl Style {
    fn class(&self) -> String {
        self.get_str("class").unwrap_or("").to_string()
    }

    fn which<A, B>(either: &Either<A, B>, selector: Selector) -> Self {
        match selector {
            Selector::IsLeft => {
                if either.is_left() {
                    Style::Selected
                } else {
                    Style::Unselected
                }
            }
            Selector::IsRight => {
                if either.is_right() {
                    Style::Selected
                } else {
                    Style::Unselected
                }
            }
        }
    }
}
