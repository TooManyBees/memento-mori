mod rules;
mod world;

use crate::rules::life;
use crate::world::{World, BOARD_HEIGHT, BOARD_WIDTH};
use nannou::prelude::*;

const CELL_SIZE: u32 = 16;

fn main() {
    nannou::app(model)
        .loop_mode(LoopMode::RefreshSync)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    world: World,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(
            BOARD_WIDTH as u32 * CELL_SIZE,
            BOARD_HEIGHT as u32 * CELL_SIZE,
        )
        .build()
        .unwrap();

    Model {
        world: World::new(),
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {

}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let (board, next_board) = model.world.this_board_and_next();
    life(board, next_board);
    model.world.swap();
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    // Turn cartesian coordinates into graphics coordinates
    let draw = app
        .draw()
        .x_y(
            (BOARD_WIDTH * CELL_SIZE as usize) as f32 * -0.5,
            (BOARD_HEIGHT * CELL_SIZE as usize) as f32 * 0.5,
        )
        .scale_y(-1.0);

    let board = model.world.board();
    for (i, cell) in board.iter().enumerate() {
        let row = i / BOARD_WIDTH;
        let col = i % BOARD_WIDTH;

        if cell.state() == true {
            draw.rect()
                .width(CELL_SIZE as f32)
                .height(CELL_SIZE as f32)
                .x_y(
                    (row * CELL_SIZE as usize) as f32 + 0.5 * CELL_SIZE as f32,
                    (col * CELL_SIZE as usize) as f32 + 0.5 * CELL_SIZE as f32,
                )
                .color(WHITE);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
