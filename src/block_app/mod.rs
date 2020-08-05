use gd_learn_001::{BLACK, GREEN, RED};
use std::collections::HashMap;
use gd_learn_001::game_engine::shapes::{Block, ShapeKind, BlockBuilder};
use gd_learn_001::game_engine::game_app::GameApp;
use gd_learn_001::game_engine::parse_block_list::parse_block_list;
use std::fs;
use ggez::{event, Context, GameResult, timer};
use gd_learn_001::game_engine::game_board::{GameBoard, PixelMap, Pixel};
use gd_learn_001::game_engine::{draw, blit_shapes};
use ggez::graphics::Canvas;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;


// most of these configurations can be loaded from config file later
pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 800;
pub const MAP_SIZE: i32 = 200;
pub const BOARD_WIDTH: f32 = 800.0;
pub const BOARD_HEIGHT: f32 = 800.0;


pub struct BlockGame {
    pub blocks: Vec<Block>,
    pub point_list: Vec<Vec<f32>>,
    pub dt: std::time::Duration,
    pub canvas: Canvas,
}

impl BlockGame {
    pub fn new(canvas: Canvas) -> Self {
        let mut point_list = vec![];
        let contents = fs::read_to_string("game_cfg")
            .expect("Something went wrong reading the file");
        BlockGame {
            blocks: parse_block_list(contents, &mut point_list),
            point_list,
            dt: std::time::Duration::new(0, 0),
            canvas
        }
    }
}

impl event::EventHandler for BlockGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(_ctx);
        let k = self.get_map_size() as f32;
        for block in &mut self.blocks {
            block.update_position(self.dt, &mut self.point_list[block.index]);
            wrap_coordinates(
                &mut self.point_list[block.index],
                k,
            );
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let bw = self.get_block_width();
        let bh = self.get_block_height();
        let map_size = self.get_map_size();
        self.point_list.iter_mut().par_bridge().for_each(|point|{
            wrap_coordinates(point, map_size as f32);
        });
        let mut b = Vec::new();
        let mut p = Vec::new();
        {
            let (_b, _p) = self.get_drawables();
            b = _b.clone();
            p = _p.clone();
        }
        let mut pixels = blit_shapes(&b, &p);
        pixels.values_mut().par_bridge().for_each(|value| {
            let (ox, oy) = wrap_points(value.point.x, value.point.y, map_size as f32);
            value.point.x = ox;
            value.point.y = oy;
        });
        draw(ctx, &pixels, bw, bh, &mut self.canvas);
        Ok(())
    }
}

impl GameApp for BlockGame {
    fn get_drawables(&self) -> (&Vec<Block>, &Vec<Vec<f32>>) {
        (&self.blocks, &self.point_list)
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
    fn get_board_width(&self) -> f32 {
        BOARD_WIDTH
    }
    fn get_board_height(&self) -> f32 {
        BOARD_HEIGHT
    }
    fn get_block_width(&self) -> f32 {
        BOARD_WIDTH / MAP_SIZE as f32
    }
    fn get_block_height(&self) -> f32 {
        BOARD_HEIGHT / MAP_SIZE as f32
    }
}

pub fn wrap_points(ix: f32, iy: f32, map_size: f32) -> (f32, f32){
    let mut ox = ix;
    let mut oy = iy;
    if ix < 0.0 { ox = ix + map_size }
    if ix >= map_size { ox = ix - map_size; }
    if iy < 0.0 { oy = iy + map_size }
    if iy >= map_size { oy = iy - map_size; }
    (ox, oy)

}

pub fn wrap_coordinates(points: &mut Vec<f32>, map_size: f32) {
    let point_pairs = if points.len() % 2 == 0 { points.len() } else { points.len() - 1 };
    for point in (0..point_pairs).step_by(2) {
        let (ox, oy) = wrap_points(points[point], points[point + 1], map_size);
        points[point] = ox;
        points[point + 1] = oy;
    }
}

