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
    brush_size: f32,
    brush_pos: Vec2,
    draw_brush: bool,
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
        brush_size: 8.0,
        brush_pos: Vec2::ZERO,
        draw_brush: false,
    }
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            simple: Some(window_event),
            ..
        } => match window_event {
            WindowEvent::Focused => app.set_loop_mode(LoopMode::RefreshSync),
            WindowEvent::Unfocused => app.set_loop_mode(LoopMode::loop_ntimes(0)),
            WindowEvent::MouseEntered => model.draw_brush = true,
            WindowEvent::MouseExited => model.draw_brush = false,
            WindowEvent::MouseMoved(pos) => model.brush_pos = pos,
            WindowEvent::MousePressed(button) => println!("Mouse pressed: {:?}", button),
            WindowEvent::MouseReleased(button) => println!("Mouse released: {:?}", button),
            WindowEvent::KeyPressed(key) => println!("Key pressed: {:?}", key),
            WindowEvent::KeyReleased(key) => println!("Key released: {:?}", key),
            WindowEvent::MouseWheel(delta, _) => println!("Scroll {:?}", delta),
            _ => {}
        },
        _ => {}
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let (board, next_board) = model.world.this_board_and_next();
    life(board, next_board);
    model.world.swap();
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    // Turn cartesian coordinates into graphics coordinates
    let mut draw = app
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

    if model.draw_brush {
        draw = draw.scale_y(-1.0).x_y(
            (BOARD_WIDTH * CELL_SIZE as usize) as f32 * 0.5,
            (BOARD_HEIGHT * CELL_SIZE as usize) as f32 * -0.5,
        );
        draw.ellipse()
            .radius(model.brush_size * CELL_SIZE as f32)
            .xy(model.brush_pos)
            .stroke_weight(2.0)
            .stroke(RED)
            .no_fill();
    }

    draw.to_frame(app, &frame).unwrap();
}
