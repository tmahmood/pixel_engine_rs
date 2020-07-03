use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use graphics::rectangle;
use graphics::math::Vec2d;
use piston::window::Position;
use gd_learn_001::{BLACK, GREEN, RED};
use graphics::types::Rectangle;
use std::collections::HashMap;
use gd_learn_001::game_engine::shapes::{Block, ShapeKind, BlockBuilder};
use gd_learn_001::game_engine::game_app::GameApp;
use image::GenericImage;
use gd_learn_001::game_engine::parse_block_list::parse_block_list;
use std::fs;


// most of these configurations can be loaded from config file later
pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 800;
pub const MAP_SIZE: i32 = 40;
pub const BOARD_WIDTH: f64 = 600.0;
pub const BOARD_HEIGHT: f64 = 600.0;
pub const BLOCK_WIDTH: f64 = BOARD_WIDTH / MAP_SIZE as f64;
pub const BLOCK_HEIGHT: f64 = BOARD_HEIGHT / MAP_SIZE as f64;


pub struct BlockGame {
    pub blocks: Vec<Block>,
    pub point_list: Vec<Vec<f64>>,
}

impl BlockGame {
    pub fn new() -> Self {
        let mut point_list = vec![];
        let contents = fs::read_to_string("game_cfg")
            .expect("Something went wrong reading the file");
        BlockGame {
            blocks: parse_block_list(contents, &mut point_list),
            point_list,
        }
    }
}

impl GameApp for BlockGame {
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

