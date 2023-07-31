use yew::prelude::*;

use crate::app::components::tile_form::TileForm;

#[derive(PartialEq, Properties)]
pub struct GameTileProps {
    pub form: TileForm,
}

#[function_component]
pub fn GameTile(props: &GameTileProps) -> Html {
    let classes = classes!("game-tile", props.form.get_class());

    html! {
        <div class={ classes }>
            <div class="game-tile-text"> { props.form } </div>
        </div>
    }
}
