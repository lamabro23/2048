use rand::distributions::WeightedIndex;
use rand::prelude::*;
use yew::prelude::*;
use yew::props;

mod components;
mod contexts;

use components::{
    button::{Button, ButtonProps},
    cell_container::CellContainer,
    game_cell::GameCell,
    game_tile::GameTile,
    topbar::TopBar,
};

use crate::app::components::tile_container::TileContainer;
use crate::app::components::tile_form::TileForm;
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

    let forms = use_state(|| vec![TileForm::Empty; 16]);

    let new_game_onclick = {
        let score = score.clone();
        let forms = forms.clone();
        move |_| {
            score.dispatch(ScoreAction::Reset);
            let mut rng = rand::thread_rng();

            let numbers = (0..16).collect::<Vec<u8>>();
            let chosen_two: Vec<u8> = numbers.choose_multiple(&mut rng, 2).cloned().collect();

            let mut new_forms = vec![TileForm::Empty; 16];

            let choices = [TileForm::Two, TileForm::Four];
            let weights = [3, 1];
            let dist = WeightedIndex::new(&weights).unwrap();
            let mut rng = thread_rng();

            let start_tile_1 = choices[dist.sample(&mut rng)];
            let start_tile_2 = match start_tile_1 {
                TileForm::Two => choices[dist.sample(&mut rng)],
                TileForm::Four => TileForm::Two,
                _ => TileForm::Empty,
            };

            new_forms[chosen_two[0] as usize] = start_tile_1;
            new_forms[chosen_two[1] as usize] = start_tile_2;

            forms.set(new_forms);

            let start_score = u32::from(start_tile_1) + u32::from(start_tile_2);
            score.dispatch(ScoreAction::Add(start_score));
        }
    };

    let new_game_btn_props = props! {
        ButtonProps<AttrValue> {
            event: new_game_onclick,
            label: "new game",
            class: "start",
        }
    };

    html! {
        <div class="page">
            <ContextProvider<ScoreContext> context={ score }>
                <TopBar/>
                <div class="game">
                    <Button<AttrValue> ..new_game_btn_props />
                    <div class="game-container">
                        <CellContainer>
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
                        </CellContainer>
                        <TileContainer>
                            <GameTile form={ forms[0] } />
                            <GameTile form={ forms[1] } />
                            <GameTile form={ forms[2] } />
                            <GameTile form={ forms[3] } />
                            <GameTile form={ forms[4] } />
                            <GameTile form={ forms[5] } />
                            <GameTile form={ forms[6] } />
                            <GameTile form={ forms[7] } />
                            <GameTile form={ forms[8] } />
                            <GameTile form={ forms[9] } />
                            <GameTile form={ forms[10] } />
                            <GameTile form={ forms[11] } />
                            <GameTile form={ forms[12] } />
                            <GameTile form={ forms[13] } />
                            <GameTile form={ forms[14] } />
                            <GameTile form={ forms[15] } />
                        </TileContainer>
                    </div>
                </div>
            </ContextProvider<ScoreContext>>
        </div>
    }
}
