use gloo::events::EventListener;
use rand::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::props;

mod components;
mod contexts;

use components::{
    button::{Button, ButtonProps},
    cell_container::CellContainer,
    game_cell::GameCell,
    game_tile::{GameTile, TileMoveEvent},
    tile_container::TileContainer,
    tile_form::TileForm,
    tile_move::TileMove,
    topbar::TopBar,
};
use contexts::score::{Score, ScoreAction, ScoreContext};

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
    let movements = use_state(|| vec![TileMove::Still; 16]);

    let new_game_onclick = {
        let score = score.clone();
        let forms = forms.clone();
        let movements = movements.clone();
        move |_| {
            score.dispatch(ScoreAction::Reset);
            let mut rng = rand::thread_rng();

            let numbers = (0..16).collect::<Vec<u8>>();
            let _chosen_two: Vec<u8> = numbers.choose_multiple(&mut rng, 2).cloned().collect();

            let mut new_forms = vec![TileForm::Empty; 16];
            let new_movements = vec![TileMove::Still; 16];

            // let choices = [TileForm::Two, TileForm::Four];
            // let weights = [3, 1];
            // let dist = WeightedIndex::new(&weights).unwrap();
            // let mut rng = thread_rng();
            //
            // let start_tile_1 = choices[dist.sample(&mut rng)];
            // let start_tile_2 = match start_tile_1 {
            //     TileForm::Two => choices[dist.sample(&mut rng)],
            //     TileForm::Four => TileForm::Two,
            //     _ => TileForm::Empty,
            // };
            //
            // new_forms[chosen_two[0] as usize] = start_tile_1;
            // new_forms[chosen_two[1] as usize] = start_tile_2;
            // TODO: temporary solution, uncomment above code after implementing movement
            new_forms[0] = TileForm::Two;

            forms.set(new_forms);
            movements.set(new_movements);

            // let start_score = u32::from(start_tile_1) + u32::from(start_tile_2);
            // score.dispatch(ScoreAction::Add(start_score));
        }
    };

    let new_game_btn_props = props! {
        ButtonProps<AttrValue> {
            event: new_game_onclick,
            label: "new game",
            class: "start",
        }
    };

    let forms_clone = forms.clone();
    let movements_clone = movements.clone();

    let another_movements_clone = movements.clone();
    let another_forms_clone = forms.clone();

    let on_tile_move = {
        let forms = another_forms_clone;
        let movements = another_movements_clone;

        move |tile_move_event: TileMoveEvent| {
            let tile_move = tile_move_event.get_tile_move();
            let direction = tile_move.get_direction();
            let length = tile_move.get_length();
            let og_position = tile_move_event.get_og_position();

            let new_position = match direction {
                "up" => (og_position.0 - length) * 4 + og_position.1,
                "down" => (og_position.0 + length) * 4 + og_position.1,
                "left" => og_position.0 * 4 + og_position.1 - length,
                "right" => og_position.0 * 4 + og_position.1 + length,
                _ => 0,
            };

            let mut new_forms = vec![TileForm::Empty; 16];
            let new_movements = vec![TileMove::Still; 16];

            new_forms[new_position] = TileForm::Two;

            forms.set(new_forms);
            movements.set(new_movements);
        }
    };

    use_effect(move || {
        let forms = forms;
        let movements = movements;

        let document = gloo::utils::document();
        let listener = EventListener::new(&document, "keypress", move |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();

            let mut new_movements = vec![TileMove::Still; 16];
            match event.key().as_str() {
                "h" => {
                    new_movements = vec![TileMove::Still; 16];
                    match forms[3] {
                        TileForm::Two => new_movements[3] = TileMove::Left3,
                        _ => new_movements[3] = TileMove::Still,
                    }
                }
                "j" => {
                    new_movements = vec![TileMove::Still; 16];
                    match forms[0] {
                        TileForm::Two => new_movements[0] = TileMove::Down3,
                        _ => new_movements[0] = TileMove::Still,
                    }
                }
                "k" => {
                    new_movements = vec![TileMove::Still; 16];
                    match forms[12] {
                        TileForm::Two => new_movements[12] = TileMove::Up3,
                        _ => new_movements[12] = TileMove::Still,
                    }
                }
                "l" => {
                    new_movements = vec![TileMove::Still; 16];
                    match forms[0] {
                        TileForm::Two => new_movements[0] = TileMove::Right3,
                        _ => new_movements[0] = TileMove::Still,
                    }
                }
                _ => {}
            }

            movements.set(new_movements);
        });
        || drop(listener)
    });

    let forms = forms_clone;
    let movements = movements_clone;

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
                            <GameTile event={ on_tile_move.clone() } position={ (0, 0) } form={ forms[0] } movement={ movements[0] } />
                            <GameTile event={ on_tile_move.clone() } position={ (0, 1) } form={ forms[1] } movement={ movements[1] } />
                            <GameTile event={ on_tile_move.clone() } position={ (0, 2) } form={ forms[2] } movement={ movements[2] } />
                            <GameTile event={ on_tile_move.clone() } position={ (0, 3) } form={ forms[3] } movement={ movements[3] } />
                            <GameTile event={ on_tile_move.clone() } position={ (1, 0) } form={ forms[4] } movement={ movements[4] } />
                            <GameTile event={ on_tile_move.clone() } position={ (1, 1) } form={ forms[5] } movement={ movements[5] } />
                            <GameTile event={ on_tile_move.clone() } position={ (1, 2) } form={ forms[6] } movement={ movements[6] } />
                            <GameTile event={ on_tile_move.clone() } position={ (1, 3) } form={ forms[7] } movement={ movements[7] } />
                            <GameTile event={ on_tile_move.clone() } position={ (2, 0) } form={ forms[8] } movement={ movements[8] } />
                            <GameTile event={ on_tile_move.clone() } position={ (2, 1) } form={ forms[9] } movement={ movements[9] } />
                            <GameTile event={ on_tile_move.clone() } position={ (2, 2) } form={ forms[10] } movement={ movements[10] } />
                            <GameTile event={ on_tile_move.clone() } position={ (2, 3) } form={ forms[11] } movement={ movements[11] } />
                            <GameTile event={ on_tile_move.clone() } position={ (3, 0) } form={ forms[12] } movement={ movements[12] } />
                            <GameTile event={ on_tile_move.clone() } position={ (3, 1) } form={ forms[13] } movement={ movements[13] } />
                            <GameTile event={ on_tile_move.clone() } position={ (3, 2) } form={ forms[14] } movement={ movements[14] } />
                            <GameTile event={ on_tile_move.clone() } position={ (3, 3) } form={ forms[15] } movement={ movements[15] } />
                        </TileContainer>
                    </div>
                </div>
            </ContextProvider<ScoreContext>>
        </div>
    }
}
