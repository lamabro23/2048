use gloo::events::EventListener;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use super::{tile_form::TileForm, tile_move::TileMove};

#[derive(PartialEq, Properties)]
pub struct GameTileProps {
    pub event: Callback<TileMoveEvent>,
    pub position: (usize, usize),
    pub form: TileForm,
    pub movement: TileMove,
}

#[function_component]
pub fn GameTile(props: &GameTileProps) -> Html {
    let x = props.position.0;
    let y = props.position.1;
    let classes = classes!(
        "game-tile",
        format!("pos-{x}-{y}"),
        props.form.get_class(),
        props.movement.get_class()
    );

    let event = props.event.clone();
    use_effect(move || {
        let element = gloo::utils::document()
            .query_selector(&format!(".pos-{x}-{y}"))
            .expect("Element not found.");

        let event = event.clone();

        let listener = EventListener::new(&element.unwrap(), "animationend", move |e| {
            let ae = e.dyn_ref::<web_sys::AnimationEvent>().unwrap();
            let tile_move_event = TileMoveEvent {
                animation_name: TileMove::try_from(ae.animation_name().as_str()).unwrap(),
                og_position: (x, y),
            };

            event.emit(tile_move_event);
            // event.emit(ae.clone());
            // info!("Element: {:?}", element_clone.as_ref().unwrap().class_name());
        });
        || drop(listener)
    });

    html! {
        <div class={ classes }>
            <div class="game-tile-text"> { props.form } </div>
        </div>
    }
}

pub struct TileMoveEvent {
    pub animation_name: TileMove,
    pub og_position: (usize, usize),
}

impl TileMoveEvent {
    pub fn get_tile_move(&self) -> TileMove {
        self.animation_name
    }

    pub fn get_og_position(&self) -> (usize, usize) {
        self.og_position
    }
}
