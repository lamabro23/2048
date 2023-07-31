use std::fmt::Display;

use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ButtonProps<L>
where
    L: PartialEq,
{
    pub event: Callback<MouseEvent>,
    pub label: L,
    pub class: Classes,
}

#[function_component]
pub fn Button<L>(props: &ButtonProps<L>) -> Html
where
    L: PartialEq + Display,
{
    let class = props.class.clone().to_string();
    html! {
        <div class={ classes!(&class) }>
            <button onclick={ &props.event }
                    class={ classes!(format!("{class}-button")) }>
                    { &props.label }
            </button>
        </div>
    }
}
