use std::{fmt::Display, marker::PhantomData, rc::Rc};

use derive_more::Deref;
use dioxus::prelude::*;
use strum::{EnumProperty, IntoEnumIterator};

pub trait SelectEnum: IntoEnumIterator + PartialEq + Clone {
    fn label(&self) -> &str;
    fn key(&self) -> impl Display {
        self.label()
    }
}

#[component]
pub fn RibbonSelector<E: SelectEnum + 'static>(
    #[props(default)] selected: Option<E>,
    #[props(default)] on_select: Callback<E>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default)] _t: PhantomData<E>,
) -> Element {
    #[derive(Deref)]
    struct Item<E> {
        #[deref]
        e: E,
        first: bool,
        last: bool,
    }

    impl<E: PartialEq + Clone + 'static> Item<E> {
        fn border_class(&self) -> impl Display {
            if self.first {
                "border rounded-l-md"
            } else if self.last {
                "border rounded-r-md"
            } else {
                "border"
            }
        }

        fn selected_class(&self, selected: &Option<E>) -> impl Display {
            if selected.as_ref().is_some_and(|s| *s == self.e) {
                Style::Selected.class()
            } else {
                Style::Unselected.class()
            }
        }
    }

    let items = {
        let last_ix = E::iter().len() - 1;
        E::iter()
            .enumerate()
            .map(move |(index, e)| Item {
                e,
                first: index == 0,
                last: index == last_ix,
            })
            .map(Rc::new)
    };

    // let onclick = |i: Item<E>| use_callback(|_: Event<MouseEvent>| i.on_click());
    rsx! {
        div { class: "flex flex-row divide-x", ..attributes,
            for i in items {
                div { key: "{i.key()}",
                    class: "{i.selected_class(&selected)} {i.border_class()}",
                    onclick: {
                        let i = i.clone();
                        move |_| on_select(i.e.clone())
                    },
                    {i.label() }
                }
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

impl Style {
    fn class(&self) -> String {
        self.get_str("class").unwrap_or("").to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use strum::{EnumIter, EnumProperty, IntoEnumIterator};

    #[derive(Debug, EnumIter, EnumProperty)]
    enum Selection {
        #[strum(props(label = "_a_"))]
        A,
        #[strum(props(label = "_b_"))]
        B,
        #[strum(props(label = "_c_"))]
        C,
    }

    #[test]
    fn enum_selection() {
        selection::<Selection>();
    }

    fn selection<E: IntoEnumIterator + EnumProperty + Debug>() {
        E::iter().for_each(|i| {
            println!("{i:?}: {}", i.get_str("label").unwrap_or("?"));
        });
    }
}
