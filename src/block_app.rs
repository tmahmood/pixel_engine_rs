use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use graphics::rectangle;
use graphics::math::Vec2d;
use piston::window::Position;
use snake_game::commands::{Commands};
use snake_game::{BLACK, GREEN};
use snake_game::game_app::GameApp;
use graphics::types::Rectangle;
use snake_game::point3d::Point3D;
use snake_game::shapes::{ShapeKind, Shape, Block};


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
}

impl App {
    pub fn new() -> Self {
        App {
            blocks: vec![
                Block::new(
                    [32.0, 32.0],
                    GREEN,
                    3.0,
                    0.0,
                )
            ]
        }
    }
}

impl GameApp for App {

    fn get_drawables(&self, args: &RenderArgs) -> Vec<(Vec<f64>, ShapeKind)> {
        // what we want to draw?!
        let mut points = Vec::new();
        for block in &self.blocks {
            points.push((block.get_shape_info(), block.get_shape_kind()));
        }
        points
    }

    fn update(&mut self, args: &UpdateArgs) {
        for block in &mut self.blocks {
            let p = block.get_shape_info();
            let [dx, dy] = block.get_movement_rate();
            block.set_shape_position(
                p[0] + (dx * args.dt as f32) as f64,
                p[1] + (dy * args.dt as f32) as f64
            );
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

impl Commands for App {
    fn run_command(cmd_str: &str) -> bool {
        true
    }
}
