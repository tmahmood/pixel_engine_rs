pub mod point3d;
pub mod game_app;
pub mod shapes;
pub mod commands;

use crate::{BLACK, GREEN, RED};
use crate::game_engine::shapes::{ShapeKind, Block};
use crate::game_engine::game_app::GameApp;
use crate::game_engine::game_board::{GameBoard, Pixel, PixelMap, draw_ellipse, draw_circle};
use std::collections::HashMap;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use opengl_graphics::{GlGraphics, OpenGL};
use piston_window::{PistonWindow as Window, WindowSettings};
use piston::{RenderArgs, Events, EventSettings, RenderEvent, UpdateEvent};
use graphics::{clear, rectangle};
use graphics::types::Color;
use crate::game_engine::game_events::GameEvents;


pub mod parse_block_list;
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

pub fn blit_shapes(shapes: &Vec<Block>, point_list: &Vec<Vec<f32>>) -> PixelMap {
    let mut pixels: PixelMap = HashMap::new();
    // draw things, we check what kind of shape we are working with
    // and draw appropriate shape
    shapes.iter().for_each(|block| {
        let k = &point_list[block.index];
        match block.shape {
            ShapeKind::Rect => {
                game_board::draw_rectangle(
                    k[0], k[1], k[2], k[3], &mut pixels,
                    Color::from(block.color),
                );
            }
            ShapeKind::Circle => {
                draw_circle(
                    k[0], k[1], k[2], &mut pixels,
                    Color::from(block.color),
                );
            }
            ShapeKind::Ellipse => {
                draw_ellipse(
                    k[0], k[1], k[2], k[3], &mut pixels,
                    Color::from(block.color),
                );
            }
            ShapeKind::Polygon => {
                let mut p: Vec<f32> = Vec::new();
                let mut n = 0;
                point_list[block.index]
                    .chunks(2)
                    .for_each(|item| {
                        p.extend(item);
                        if p.len() == 4 {
                            game_board::draw_line(
                                p[0], p[1], p[2], p[3], &mut pixels,
                                Color::from(block.color),
                            );
                            p = vec![p[2], p[3]];
                            n += 1;
                        }
                    });
                game_board::draw_line(
                    point_list[block.index][0],
                    point_list[block.index][1],
                    p[0], p[1],
                    &mut pixels,
                    Color::from(block.color),
                );
            }
            ShapeKind::Line => {
                game_board::draw_line(
                    k[0], k[1], k[2], k[3], &mut pixels,
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


pub fn game_loop<T: GameApp + GameEvents>(mut app: T, mut game_data: GameEngineData) {
    use graphics::*;
    let mut events = Events::new(EventSettings::new());
    // drawing context
    while let Some(e) = events.next(&mut game_data.window) {
        if let Some(args) = e.render_args() {
            // get all the drawable
            let pixels = app.render(&args);
            let block_width = app.get_block_width();
            let block_height = app.get_block_height();
            let base_rect = rectangle::rectangle_by_corners(
                0.0, 0.0,
                block_width as f64, block_height as f64,
            );
            // doing drawing stuffS
            game_data.gl.draw(args.viewport(), |c, gl| {
                // Clear the screen.
                clear(BLACK, gl);
                pixels.values().for_each(|pixel| {
                    let transform = c.transform.trans(
                        (pixel.point.x * block_width) as f64,
                        (pixel.point.y * block_height) as f64
                    );
                    rectangle(pixel.color, base_rect, transform, gl);
                });
            });
        }
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
