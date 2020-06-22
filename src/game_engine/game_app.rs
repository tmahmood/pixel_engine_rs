use piston::input::{RenderArgs, UpdateArgs};
use graphics::types::Rectangle;
use crate::game_engine::shapes::Block;

pub trait GameApp {
    fn get_drawables(&self, args: &RenderArgs) -> (&Vec<Block>, &Vec<Vec<f64>>);
    fn update(&mut self, args: &UpdateArgs);
    fn get_window_width(&self) -> i32;
    fn get_window_height(&self) -> i32;
    fn get_map_size(&self) -> i32;
    fn get_board_width(&self) -> f64;
    fn get_board_height(&self) -> f64;
    fn get_block_width(&self) -> f64;
    fn get_block_height(&self) -> f64;
}

