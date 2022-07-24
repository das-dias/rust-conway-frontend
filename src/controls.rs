use yew::prelude::{function_component, html, Callback, Properties, MouseEvent};

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct ControlsState {
    pub class: String,
    pub handle_next_generation: Callback<MouseEvent>,
    pub handle_previous_generation: Callback<MouseEvent>,
    pub handle_reset_board: Callback<MouseEvent>,
}

#[function_component(Controls)]
pub fn controls(props: &ControlsState) -> Html {

    html! {
        <div class={props.class.clone()}>
            <button 
                onclick={(&props).handle_next_generation.clone()}
                style={
                    "background-color: #555;
                    color: #fff;
                    border-radius: 5%;
                    width: 116px;
                    height: 46px;
                    border-width: 2px;
                    border-color: #fff;
                    margin: 10px;
                    font-weight: bold;"
                }
            >
                {"Next generation"}
            </button>
            <button 
                onclick={(&props).handle_previous_generation.clone()}
                style={
                    "background-color: #555;
                    color: #fff;
                    border-radius: 5%;
                    width: 116px;
                    height: 46px;
                    border-width: 2px;
                    border-color: #fff;
                    margin: 10px;
                    font-weight: bold;"
                }
            >
                {"Previous generation"}
            </button>
            <button 
                onclick={(&props).handle_reset_board.clone()}
                style={
                    "background-color: #555;
                    color: #fff;
                    border-radius: 5%;
                    width: 116px;
                    height: 46px;
                    border-width: 2px;
                    border-color: #fff;
                    margin: 10px;
                    font-weight: bold;"
                }

            >
                {"Reset board"}
            </button>
        </div>
    }
}