use yew::prelude::{function_component, html, Callback, Properties, MouseEvent};

#[derive( PartialEq, Debug, Properties)]
pub struct CellProps {
    pub alive: bool,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    let color = if (&props).alive {"black"} else {"white"};
    html! {
        <button
            class="cell"
            onclick = {(&props).onclick.clone()}
            style={format!(
                "background-color: {color}; 
                border-radius: 30%; 
                width: 66px; 
                height: 66px; 
                border-width: 2px;
                border-color: #555", 
                color=color
            )}
        >
        </button>
    }
}