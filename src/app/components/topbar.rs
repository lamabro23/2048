use yew::prelude::*;
use yew::props;

use crate::app::components::score_counter::{ScoreCounter, ScoreCounterProps};
use crate::app::ScoreContext;

#[derive(PartialEq, Properties)]
pub struct TopBarProps {}

#[function_component]
pub fn TopBar(_props: &TopBarProps) -> Html {
    let score = use_context::<ScoreContext>().unwrap();

    let score_counter_props = props! {
        ScoreCounterProps<AttrValue> {
            label: "score",
            class: "score",
            value: score.current,
        }
    };

    let best_counter_props = props! {
        ScoreCounterProps<AttrValue> {
            label: "best",
            class: "best",
            value: score.best,
        }
    };

    html! {
        <div class="top-bar">
            <ScoreCounter<AttrValue> ..score_counter_props />
            <ScoreCounter<AttrValue> ..best_counter_props />
        </div>
    }
}
