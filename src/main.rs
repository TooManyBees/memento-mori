use nannou::prelude::*;

#[derive(Copy, Clone, Debug)]
struct Cell {
    state: bool,
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

    Model {
        state_a: [Cell { state: false }; BOARD_WIDTH * BOARD_HEIGHT],
        state_b: [Cell { state: true }; BOARD_WIDTH * BOARD_HEIGHT],
        current_board: CurrentBoard::A,
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {

}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // model.swap();
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    // Turn cartesian coordinates into graphics coordinates
    let draw = app.draw()
        .x_y((BOARD_WIDTH * CELL_SIZE as usize) as f32 * -0.5, (BOARD_HEIGHT * CELL_SIZE as usize) as f32 * 0.5)
        .scale_y(-1.0);

    let board: &Board = model.board();
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
