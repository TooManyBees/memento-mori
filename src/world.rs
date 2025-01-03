use nannou::rand;

pub const BOARD_WIDTH: usize = 40;
pub const BOARD_HEIGHT: usize = 40;

#[derive(Copy, Clone, Default)]
pub struct Cell {
    state: bool,
}

impl Cell {
    pub fn new(state: bool) -> Self {
        Cell { state }
    }

    pub fn state(&self) -> bool {
        self.state
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.state {
            true => 1u8,
            false => 0u8,
        }
        .fmt(fmt)
    }
}

pub type Board = [Cell; BOARD_WIDTH * BOARD_HEIGHT];

#[derive(Clone, Debug)]
pub struct World {
    state_a: Board,
    state_b: Board,
    current_board: CurrentBoard,
}

impl World {
    pub fn new() -> Self {
        let mut state_a = [Cell::new(false); BOARD_WIDTH * BOARD_HEIGHT];
        let state_b = state_a.clone();
        for cell in &mut state_a {
            cell.state = rand::random::<bool>();
        }

        World {
            state_a,
            state_b,
            current_board: CurrentBoard::A,
        }
    }

    pub fn board(&self) -> &Board {
        match self.current_board {
            CurrentBoard::A => &self.state_a,
            CurrentBoard::B => &self.state_b,
        }
    }

    pub fn this_board_and_next(&mut self) -> (&Board, &mut Board) {
        match self.current_board {
            CurrentBoard::A => (&self.state_a, &mut self.state_b),
            CurrentBoard::B => (&self.state_b, &mut self.state_a),
        }
    }

    pub fn swap(&mut self) {
        self.current_board = match self.current_board {
            CurrentBoard::A => CurrentBoard::B,
            CurrentBoard::B => CurrentBoard::A,
        };
    }
}

#[derive(Copy, Clone, Debug)]
enum CurrentBoard {
    A,
    B,
}
