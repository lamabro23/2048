use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct TileContainerProps {
    pub children: Children,
}

#[function_component]
pub fn TileContainer(props: &TileContainerProps) -> Html {
    html! {
        <div class="tile-container">
            { props.children.clone() }
        </div>
    }
}
