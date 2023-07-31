use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct CellContainerProps {
    pub children: Children,
}

#[function_component]
pub fn CellContainer(props: &CellContainerProps) -> Html {
    html! {
        <div class="cell-container">
            { props.children.clone() }
        </div>
    }
}
