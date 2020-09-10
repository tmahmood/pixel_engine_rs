use crate::game_engine::shapes::Block;
use crate::game_engine::game_board::PixelMap;

/// GameDataModel implements engine independent game data
/// It should never bound to any specific game library
pub trait GameDataModel {
    fn get_drawables(&self) -> (Vec<Block>, &Vec<Vec<f32>>);
    fn get_window_width(&self) -> i32;
    fn get_window_height(&self) -> i32;
    fn get_map_size(&self) -> i32;
    fn get_board_width(&self) -> f32;
    fn get_board_height(&self) -> f32;
    fn get_block_width(&self) -> f32;
    fn get_block_height(&self) -> f32;
}
