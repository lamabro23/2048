use std::fmt::Display;

use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ScoreCounterProps<L>
where
    L: PartialEq,
{
    pub label: L,
    pub class: Classes,
    pub value: u32,
}

#[function_component]
pub fn ScoreCounter<L>(props: &ScoreCounterProps<L>) -> Html
where
    L: PartialEq + Display,
{
    let label = format!("{}", &props.label);
    let class = classes!(props.class.clone(), "display");

    html! {
        <div class={ class }>
            <div class="label-container">
                <span class="label">{ label }</span>
            </div>
            <div class="value-container">
                <span class="value">{ props.value }</span>
            </div>
        </div>
    }
}
