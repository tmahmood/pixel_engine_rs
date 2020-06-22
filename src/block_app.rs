use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use graphics::rectangle;
use graphics::math::Vec2d;
use piston::window::Position;
use snake_game::{BLACK, GREEN, RED};
use graphics::types::Rectangle;
use std::collections::HashMap;
use snake_game::game_engine::shapes::{Block, ShapeKind};
use snake_game::game_engine::game_app::GameApp;


// most of these configurations can be loaded from config file later
pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 800;
pub const MAP_SIZE: i32 = 40;
pub const BOARD_WIDTH: f64 = 600.0;
pub const BOARD_HEIGHT: f64 = 600.0;
pub const BLOCK_WIDTH: f64 = BOARD_WIDTH / MAP_SIZE as f64;
pub const BLOCK_HEIGHT: f64 = BOARD_HEIGHT / MAP_SIZE as f64;


pub struct App {
    pub blocks: Vec<Block>,
    pub point_list: Vec<Vec<f64>>,
}

impl App {
    pub fn new() -> Self {
        let mut point_list = vec![];
        App {
            blocks: vec![
                Block::new(
                    vec![32.0, 32.0],
                    GREEN,
                    3.0,
                    0.0,
                    ShapeKind::Rect,
                    &mut point_list,
                ),
                Block::new(
                    vec![10.0, 14.0],
                    RED,
                    1.0, 1.0,
                    ShapeKind::Ellipse,
                    &mut point_list,
                ),
                Block::new(
                    vec![30.0, 30.0, 35.0, 35.0, 1.0],
                    RED,
                    0.0,
                    0.0,
                    ShapeKind::Line,
                    &mut point_list
                ),
                Block::new(
                    vec![
                        20.0, 10.0,
                        25.0, 15.0,
                        20.0, 15.0,
                    ],
                    GREEN,
                    0.0,
                    1.0,
                    ShapeKind::Polygon,
                    &mut point_list
                )
            ],
            point_list,
        }
    }
}

impl GameApp for App {
    fn get_drawables(&self, args: &RenderArgs) -> (&Vec<Block>, &Vec<Vec<f64>>) {
        (&self.blocks, &self.point_list)
    }

    fn update(&mut self, args: &UpdateArgs) {
        for block in &mut self.blocks {
            block.update_position(args, &mut self.point_list[block.index]);
        }
    }

    fn get_window_width(&self) -> i32 {
        WINDOW_WIDTH
    }
    fn get_window_height(&self) -> i32 {
        WINDOW_HEIGHT
    }
    fn get_map_size(&self) -> i32 {
        MAP_SIZE
    }
    fn get_board_width(&self) -> f64 {
        BOARD_WIDTH
    }
    fn get_board_height(&self) -> f64 {
        BOARD_HEIGHT
    }
    fn get_block_width(&self) -> f64 {
        BLOCK_WIDTH
    }
    fn get_block_height(&self) -> f64 {
        BLOCK_HEIGHT
    }
}

