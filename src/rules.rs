
fn cell_purgatory(cell_index: i64, board: Vec<bool>, board_size: i64) -> bool {
    let mut alive_neighbours = 0;
    for i in -1..2 {
        for j in -1..2 {
            let neighbour_index = (cell_index + i) + (j * board_size);
            if neighbour_index >= 0 && neighbour_index < board.len() as i64 {
                if board[neighbour_index as usize] {
                    alive_neighbours += 1;
                }
            }
        }
    }
    if (board[cell_index as usize] && alive_neighbours < 2) || (board[cell_index as usize] && alive_neighbours > 3) { 
        false 
    } else {
        if !board[cell_index as usize] && alive_neighbours == 3 { 
            true 
        } else { 
            board[cell_index as usize]
        }
    } // no semicolon ot reeturn the value
}

pub struct NewBoardState {
    pub board: Vec<bool>,
    pub board_changed: bool,
}

pub fn next_generation(current_board: Vec<bool>, board_size: i64) -> NewBoardState {
    let mut changed = false;
    let mut cell_index = 0;
    let new_board = current_board.clone().into_iter().map( move |cell| {
        let new_cell = cell_purgatory(cell_index, current_board.clone(), board_size);
        cell_index += 1;
        changed = changed || (new_cell != cell);
        return new_cell;
    }).collect();
    NewBoardState {
        board: new_board,
        board_changed: changed,
    }
}