use nannou::prelude::*;
use nannou::rand;

#[derive(Copy, Clone, Default)]
struct Cell {
    state: bool,
}
impl std::fmt::Debug for Cell {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.state {
            true => 1u8,
            false => 0u8,
        }.fmt(fmt)
    }
}

const BOARD_WIDTH: usize = 40;
const BOARD_HEIGHT: usize = 40;
const CELL_SIZE: u32 = 16;

type Board = [Cell; BOARD_WIDTH * BOARD_HEIGHT];

#[derive(Copy, Clone, Debug)]
enum CurrentBoard {
    A,
    B,
}

#[derive(Clone, Debug)]
struct Model {
    state_a: Board,
    state_b: Board,
    current_board: CurrentBoard,
}

impl Model {
    fn board(&self) -> &Board {
        match self.current_board {
            CurrentBoard::A => &self.state_a,
            CurrentBoard::B => &self.state_b,
        }
    }

    fn this_board_and_next(&mut self) -> (&Board, &mut Board) {
        match self.current_board {
            CurrentBoard::A => (&self.state_a, &mut self.state_b),
            CurrentBoard::B => (&self.state_b, &mut self.state_a),
        }
    }

    fn swap(&mut self) {
        self.current_board = match self.current_board {
            CurrentBoard::A => CurrentBoard::B,
            CurrentBoard::B => CurrentBoard::A,
        };
    }
}

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::RefreshSync)
        .event(event)
        .update(update)
        .view(view)
        .run();
}


fn model(app: &App) -> Model {
    app.new_window()
        .size(BOARD_WIDTH as u32 * CELL_SIZE, BOARD_HEIGHT as u32 * CELL_SIZE)
        .build()
        .unwrap();

    let mut state_a = [Cell { state: false }; BOARD_WIDTH * BOARD_HEIGHT];
    let state_b = state_a.clone();
    for cell in &mut state_a {
        cell.state = rand::random::<bool>();
    }

    Model {
        state_a,
        state_b,
        current_board: CurrentBoard::A,
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {

}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let (board, next_board) = model.this_board_and_next();
    game_of_life(board, next_board);
    model.swap();
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    // Turn cartesian coordinates into graphics coordinates
    let draw = app.draw()
        .x_y((BOARD_WIDTH * CELL_SIZE as usize) as f32 * -0.5, (BOARD_HEIGHT * CELL_SIZE as usize) as f32 * 0.5)
        .scale_y(-1.0);

    let board = model.board();
    for (i, cell) in board.iter().enumerate() {
        let row = i / BOARD_WIDTH;
        let col = i % BOARD_WIDTH;

        if cell.state == true {
            draw.rect()
                .width(CELL_SIZE as f32)
                .height(CELL_SIZE as f32)
                .x_y((row * CELL_SIZE as usize) as f32 + 0.5 * CELL_SIZE as f32, (col * CELL_SIZE as usize) as f32 + 0.5 * CELL_SIZE as f32)
                .color(WHITE);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn game_of_life(board: &Board, next_board: &mut Board) {
    for row in 0..BOARD_HEIGHT {
        for col in 0..BOARD_WIDTH {
            let next_cell_state = game_of_life_next_cell_state(board, row, col);
            next_board[row * BOARD_WIDTH + col] = Cell { state: next_cell_state };
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Delta {
    LT,
    EQ,
    GT,
}

fn game_of_life_next_cell_state(board: &Board, row: usize, col: usize) -> bool {
    let mut live_neighbors = 0;
    let idx = row * BOARD_WIDTH + col;

    for dy in [Delta::LT, Delta::EQ, Delta::GT].into_iter() {
        if (dy == Delta::LT && row == 0) || (dy == Delta::GT && (row + 1) == BOARD_HEIGHT) {
            continue;
        }

        for dx in [Delta::LT, Delta::EQ, Delta::GT].into_iter() {
            if (dx == Delta::LT && col == 0) || (dx == Delta::GT && (col + 1) == BOARD_WIDTH) || (dx == Delta::EQ && dy == Delta::EQ) {
                continue;
            }

            let neighbor_idx = match dy {
                Delta::LT => row - 1,
                Delta::EQ => row,
                Delta::GT => row + 1,
            } * BOARD_WIDTH + match dx {
                Delta::LT => col - 1,
                Delta::EQ => col,
                Delta::GT => col + 1,
            };

            if board[neighbor_idx].state == true {
                live_neighbors += 1;
            }
        }
    }

    match (board[idx].state, live_neighbors) {
        (true, x) if x < 2 => false,
        (true, 2) | (true, 3) => true,
        (true, x) if x > 3 => false,
        (false, 3) => true,
        (whatever, _) => whatever,
    }
}
