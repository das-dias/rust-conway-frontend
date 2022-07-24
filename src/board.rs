use yew::prelude::{function_component, html, Callback, Properties, MouseEvent};

use crate::cell::*;

#[derive( PartialEq, Debug, Properties)]
pub struct BoardState {
    pub class: String,
    pub cells: Vec<bool>,
    pub board_size: i64,
    pub cell_onclick: Callback<i64>,
}

#[function_component(Board)]
pub fn board(props: &BoardState ) -> Html {
    let cells = props.cells.clone();
    let board_size = props.board_size;

    let mut html_cells = vec![];
    for i in 0..board_size {
        let mut row = vec![];
        for j in 0..board_size {     
            let cell_index = i * board_size + j;
            let cell_onclick_clone = props.cell_onclick.clone();
            let handle_cell_click = {Callback::from( move |_:MouseEvent| cell_onclick_clone.emit(cell_index))};
            row.push( html! {
                <Cell
                    alive={cells[cell_index as usize]}
                    onclick={handle_cell_click}
                    key={cell_index}
                />
            });
        }
        html_cells.push( html! {
            <div class="board-row" key={format!("row-{}",i)}>
                {row}
            </div>
        });
    }

    html! {
        <div class={props.class.clone()}>
            {html_cells}
        </div>
    }
}