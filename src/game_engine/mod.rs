pub mod point3d;
pub mod game_app;
pub mod shapes;
pub mod commands;

use crate::{BLACK, GREEN, RED};
use crate::game_engine::shapes::{ShapeKind, Block};
use crate::game_engine::game_app::GameApp;
use ggez::{ContextBuilder, Context, GameResult, GameError};
use ggez::event::EventsLoop;
use ggez::graphics;
use ggez::graphics::{DrawMode, Color, Rect, Mesh, Canvas};
use ggez::nalgebra::{Point2, Point};
use crate::game_engine::game_board::{GameBoard, Pixel, PixelMap, draw_ellipse, draw_circle};
use std::collections::HashMap;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};


pub mod parse_block_list;
pub mod game_board;

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
                    Color::from(block.color)
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

pub fn draw(context: &mut Context, pixels: &PixelMap, block_width: f32, block_height: f32, canvas: &mut Canvas) {
    graphics::clear(context, [0.1, 0.2, 0.3, 1.0].into());
    // set canvas
    graphics::set_canvas(context, Some(canvas));
    // clear the screen
    graphics::clear(context, [0.1, 0.2, 0.3, 1.0].into());
    let rect = Rect::new(
        0.0, 0.0,
        block_width, block_height,
    );
    // drawing all pixels
    pixels.values().for_each(|value| {
        // create rectangle first
        let n = graphics::Mesh::new_rectangle(
            context,
            DrawMode::fill(),
            rect,
            value.color,
        );
        // draw it
        graphics::draw(context, &n.unwrap(),
                       (Point2::new(
                           value.point.x * block_width,
                           value.point.y * block_height),)
        );
    });
    graphics::set_canvas(context, None);
    graphics::draw(
        context,
        canvas, (Point2::new(0.0, 0.0), ),
    );
    graphics::present(context);
}

