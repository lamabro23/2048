use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct GameBoardProps {
    pub children: Children,
}

#[function_component]
pub fn GameBoard(props: &GameBoardProps) -> Html {
    html! {
        <div class="game-board-box">
            <div class="game-board">
                { props.children.clone() }
            </div>
        </div>
    }
}
