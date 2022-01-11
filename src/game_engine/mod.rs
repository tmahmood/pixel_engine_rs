pub mod point3d;
pub mod game_data_model;
pub mod shapes;
pub mod commands;

use crate::{BLACK, GREEN, RED};
use crate::game_engine::shapes::{ShapeKind, Block};
use crate::game_engine::game_data_model::GameDataModel;
use crate::game_engine::game_board::{GameBoard, Pixel, PixelMap};
use std::collections::HashMap;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use opengl_graphics::{GlGraphics, OpenGL};
use piston_window::{PistonWindow as Window, WindowSettings};
use piston::{RenderArgs, Events, EventSettings, RenderEvent, UpdateEvent, PressEvent, ReleaseEvent, ResizeEvent, Button, Key};
use graphics::{clear, rectangle};
use graphics::types::Color;
use crate::game_engine::game_events::PistonGameEvents;


pub mod game_board;
pub mod game_events;

pub struct GameEngineData {
    gl: GlGraphics,
    window: Window,
}

pub fn init_game_engine(size: [f64; 2], opengl: OpenGL) -> GameEngineData {
    let window = WindowSettings::new("Game", size)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let gl = GlGraphics::new(opengl);
    GameEngineData {
        gl,
        window,
    }
}

pub fn draw_shapes(shapes: &Vec<Block>, point_list: &Vec<Vec<f32>>) -> PixelMap {
    let mut pixels: PixelMap = HashMap::new();
    shapes.iter().for_each(|block| {
        let k = &point_list[block.index];
        match block.shape {
            ShapeKind::Rect => {
                game_board::draw_rectangle(
                    block.x + k[0], block.y + k[1], k[2], k[3], &mut pixels,
                    Color::from(block.color),
                );
            }
            ShapeKind::Circle => {
                game_board::draw_circle(
                    block.x + k[0], block.y + k[1], k[2], &mut pixels,
                    Color::from(block.color),
                );
            }
            ShapeKind::Ellipse => {
                game_board::draw_ellipse(
                    block.x + k[0], block.y + k[1], k[2], k[3], &mut pixels,
                    Color::from(block.color),
                );
            }
            ShapeKind::Polygon => {
                game_board::draw_polygon(k, &block, &mut pixels)
            }
            ShapeKind::Line => {
                game_board::draw_line(
                    block.x + k[0], block.y + k[1],
                    block.x + k[2], block.y + k[3],
                    &mut pixels,
                    Color::from(block.color),
                );
            }
            ShapeKind::None => {
                panic!("This should not happen")
            }
        };
    });
    pixels
}

pub fn game_loop<T: GameDataModel + PistonGameEvents>(mut app: T, mut game_data: GameEngineData) {
    use graphics::*;
    let mut events = Events::new(EventSettings::new());
    // drawing context
    while let Some(e) = events.next(&mut game_data.window) {
        if let Some(args) = e.render_args() {
            // get all the drawable
            let pixels = app.update_game_board(&args);
            let block_width = app.get_block_width();
            let block_height = app.get_block_height();
            let board_width = app.get_board_width();
            let board_height = app.get_board_height();
            let base_rect = rectangle::rectangle_by_corners(
                0.0, 0.0,
                block_width as f64, block_height as f64,
            );
            const GRID_COLOR: [f32; 4] = [0.3, 0.3, 0.3, 0.3];
            // doing drawing stuffS
            game_data.gl.draw(args.viewport(), |c, gl| {
                // Clear the screen.
                clear(BLACK, gl);
                let s = (block_width * 2.0) as i32;
                for ix in (s..board_width as i32).step_by(s as usize) {
                    let l = [ix as f64, 0.0, ix as f64, board_height as f64];
                    line(GRID_COLOR, 1.0, l, c.transform, gl);
                }
                let s = (block_height * 2.0) as i32;
                for iy in (s..board_height as i32).step_by(s as usize)  {
                    let l = [0.0, iy as f64, board_height as f64, iy as f64];
                    line(GRID_COLOR, 1.0, l, c.transform, gl);
                }
                pixels.values().for_each(|pixel| {
                    let transform = c.transform.trans(
                        (pixel.point.x * block_width) as f64,
                        (pixel.point.y * block_height) as f64
                    );
                    rectangle(pixel.color, base_rect, transform, gl);
                });
            });
        }
        if let Some(button) = e.press_args() {
            app.handle_press_events(&button);
        }
        if let Some(button) = e.release_args() {
            app.handle_release_events(&button);
        }
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
