use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct BoardTileProps {}

#[function_component]
pub fn GameCell(_props: &BoardTileProps) -> Html {
    html! {
        <div class="board-tile">
        </div>
    }
}
