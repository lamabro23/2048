use log::info;
use wasm_logger;
use yew::prelude::*;
use yew::props;

mod components;
mod contexts;

use components::{
    button::{Button, ButtonProps},
    game_board::GameBoard,
    game_cell::GameCell,
    topbar::TopBar,
};

use crate::app::contexts::score::{Score, ScoreAction, ScoreContext};

#[function_component]
pub fn App() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    let current = use_state(|| 0);
    let best = use_state(|| 0);

    let score = use_reducer(|| Score {
        current: *current,
        best: *best,
    });

    let new_game_onclick = {
        let score = score.clone();
        Callback::from(move |_| {
            score.dispatch(ScoreAction::Reset);
        })
    };

    let new_game_btn_props = props! {
        ButtonProps<AttrValue> {
            event: new_game_onclick,
            label: "new game",
            class: "start",
        }
    };

    let tmp_inc_score = {
        let score = score.clone();
        Callback::from(move |_| {
            score.dispatch(ScoreAction::Increment);
        })
    };

    let tmp_inc_btn_props = props! {
        ButtonProps<AttrValue> {
            event: tmp_inc_score,
            label: "+1",
            class: "start",
        }
    };

    html! {
        <div class="page">
            <ContextProvider<ScoreContext> context={ score }>
                <TopBar/>
                <div class="game">
                <Button<AttrValue> ..tmp_inc_btn_props />
                <Button<AttrValue> ..new_game_btn_props />
                <GameBoard>
                    <GameCell />
                    <GameCell />
                    <GameCell />
                    <GameCell />
                    <GameCell />
                    <GameCell />
                    <GameCell />
                    <GameCell />
                    <GameCell />
                    <GameCell />
                    <GameCell />
                    <GameCell />
                    <GameCell />
                    <GameCell />
                    <GameCell />
                    <GameCell />
                </GameBoard>
                </div>
            </ContextProvider<ScoreContext>>
        </div>
    }
}
