#![allow(unused_doc_comments)]
// mods
mod rules;
mod controls;
mod board;
mod cell;

// uses
use yew::prelude::*;
use rules::{NewBoardState, next_generation};
use controls::*;
use board::*;

#[derive(Clone, PartialEq, Debug, Properties)]
struct State {
    generation: i64,
    board_changed: bool,
    board: Vec<bool>,
    board_size: i64,
    history: Vec<Vec<bool>>,
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| State {
        generation: 0,
        board_changed: false,
        board_size: 9,
        board: vec![false; 81],
        history: vec![vec![false; 81],],
    });

    let handle_cell_click = {
        let state = state.clone();
        Callback::from(move |cell_index: i64| {
            let mut new_board = state.board.clone();
            new_board[cell_index as usize] = !new_board[cell_index as usize];
            state.set(State {
                generation: state.generation,
                board_changed: false,
                board: new_board,
                board_size: state.board_size,
                history: state.history.clone(),
            })
        })
    };

    let handle_next_generation = {
        let state = state.clone();
        Callback::from(move |_| {
            let new_board_state: NewBoardState = next_generation(state.board.clone(), state.board_size);
            let mut new_history = state.history.clone();
            new_history.push(state.board.clone());
            state.set(State {
                generation: state.generation + 1,
                board_changed: new_board_state.board_changed,
                board: new_board_state.board.clone(),
                board_size: state.board_size,
                history: new_history.clone(),
            })
        })
    };

    let handle_previous_generation = {
        let state = state.clone();
        Callback::from(move |_| {
            let mut history = state.history.clone();
            let generation = if state.generation - 1 < 0 {
                0
            } else {
                state.generation - 1
            };

            let old_board = if history.len() > 1 {history.pop().unwrap()} else {state.board.clone()};
            state.set(State {
                generation: generation,
                board_changed: false,
                board: old_board,
                board_size: state.board_size,
                history: history,
            })
        })
    };

    let handle_reset_board = {
        let state = state.clone();
        Callback::from(move |_: MouseEvent| {
            state.set(State {
                generation: 0,
                board_changed: false,
                board: vec![false; 81],
                board_size: state.board_size,
                history: vec![],
            })
        })
    };

    html! {
        <div class="App">
            <header class="App-header">
                <h1>{"Conway's Game of Life"}</h1>
                <h2>{"Generation: "}{state.generation}</h2>
                <h2>{"Board size: "}{state.board_size}{" by "}{state.board_size}</h2>
                <div class="App-controls">
                    <Controls
                        class="controls"
                        handle_next_generation={handle_next_generation}
                        handle_previous_generation={handle_previous_generation}
                        handle_reset_board={handle_reset_board}
                        key={"controls"}
                    />
                </div>
                <div class="App-board">
                    <Board 
                        class="board"
                        cells={state.board.clone()}
                        board_size={state.board_size}
                        cell_onclick={handle_cell_click}
                        key= {"board"}
                    />
                </div>
            </header>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
