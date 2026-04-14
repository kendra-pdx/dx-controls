use std::{fmt::Display, str::FromStr};

use dioxus::prelude::*;
use num_traits::Num;
use spacial::prelude::TwizzleXY;

pub type XY<T> = spacial::prelude::Vec2<T>;

#[component]
pub fn XyInput<T>(
    xy: ReadSignal<XY<T>>,
    #[props(default)] onchange: Callback<XY<T>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element
where
    T: PartialEq + Copy + Num + Display + FromStr + 'static,
{
    enum Field {
        X,
        Y,
    }

    let mut x = use_memo(move || xy().x());
    let mut y = use_memo(move || xy().y());

    let onchange = move |f| {
        move |e: FormEvent| {
            if let Ok(t) = e.value().parse() {
                match f {
                    Field::X => x.set(t),
                    Field::Y => y.set(t),
                };
                let xy = [x(), y()];
                onchange.call(xy);
            }
        }
    };

    rsx! {
       div { class: "flex flex-row gap-2", ..attributes,
           div { class: "flex flex-col",
               label { "X" }
               input { type: "number",
                   value: x().to_string(), onchange: onchange(Field::X)
               }
           }

           div { class: "flex flex-col",
               label { "Y" }
               input { type: "number",
                   value: y().to_string(), onchange: onchange(Field::Y)
               }
           }
       }
    }
}
