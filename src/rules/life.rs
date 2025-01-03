use crate::model::{Board, Cell, BOARD_HEIGHT, BOARD_WIDTH};

pub fn life(board: &Board, next_board: &mut Board) {
    for row in 0..BOARD_HEIGHT {
        for col in 0..BOARD_WIDTH {
            let next_cell_state = next_cell_state(board, row, col);
            next_board[row * BOARD_WIDTH + col] = Cell::new(next_cell_state);
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Delta {
    LT,
    EQ,
    GT,
}

fn next_cell_state(board: &Board, row: usize, col: usize) -> bool {
    let mut live_neighbors = 0;
    let idx = row * BOARD_WIDTH + col;

    for dy in [Delta::LT, Delta::EQ, Delta::GT].into_iter() {
        if (dy == Delta::LT && row == 0) || (dy == Delta::GT && (row + 1) == BOARD_HEIGHT) {
            continue;
        }

        for dx in [Delta::LT, Delta::EQ, Delta::GT].into_iter() {
            if (dx == Delta::LT && col == 0)
                || (dx == Delta::GT && (col + 1) == BOARD_WIDTH)
                || (dx == Delta::EQ && dy == Delta::EQ)
            {
                continue;
            }

            let neighbor_idx = match dy {
                Delta::LT => row - 1,
                Delta::EQ => row,
                Delta::GT => row + 1,
            } * BOARD_WIDTH
                + match dx {
                    Delta::LT => col - 1,
                    Delta::EQ => col,
                    Delta::GT => col + 1,
                };

            if board[neighbor_idx].state() == true {
                live_neighbors += 1;
            }
        }
    }

    match (board[idx].state(), live_neighbors) {
        (true, x) if x < 2 => false,
        (true, 2) | (true, 3) => true,
        (true, x) if x > 3 => false,
        (false, 3) => true,
        (whatever, _) => whatever,
    }
}
