use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct BoardRowProps {
    pub children: Children,
}

#[function_component]
pub fn BoardRow(props: &BoardRowProps) -> Html {
    html! {
        <div class="board-row">
            { props.children.clone() }
        </div>
    }
}
