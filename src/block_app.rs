use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};
use graphics::rectangle;
use graphics::math::Vec2d;
use piston::window::Position;
use crate::game_engine::commands::Commands;
use crate::{BLACK, GREEN};
use crate::game_app::GameApp;

pub struct Block {
    position: Position,
    color: [f32; 4],
}

impl Block {
    pub fn get_drawing_props(&self) -> ([f32; 4], (f64, f64)) {
        (self.color, (self.position.x as f64 * BLOCK_WIDTH , self.position.y as f64 * BLOCK_HEIGHT))
    }
}

pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 800;
pub const MAP_SIZE: i32 = 80;

pub const BOARD_WIDTH: f64 = 600.0;
pub const BOARD_HEIGHT: f64 = 600.0;
pub const BLOCK_WIDTH: f64 = BOARD_WIDTH / MAP_SIZE as f64;
pub const BLOCK_HEIGHT: f64 = BOARD_HEIGHT / MAP_SIZE as f64;

pub struct App {
    pub gl: GlGraphics,
    pub blocks: Vec<Block>,
}

impl App {
    pub fn new(gl: GlGraphics) -> Self {
        App {
            gl,
            blocks: Vec::new()
        }
    }
}

impl GameApp for App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let square = rectangle::rectangle_by_corners(0.0, 0.0, BLOCK_WIDTH, BLOCK_HEIGHT);
        let background = rectangle::rectangle_by_corners(0.0, 0.0, BOARD_WIDTH, BOARD_HEIGHT);
        let blocks = &self.blocks;
        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
            rectangle(GREEN, background, c.transform.trans(0.0, 0.0), gl);
            blocks.iter().for_each(|block| {
                let (color, (x, y)) = block.get_drawing_props();
                let transform = c.transform.trans(x, y);
                rectangle(color, square, transform, gl);
            });
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
    }

}

impl Commands for App {
    fn run_command(cmd_str: &str) -> bool {
        true
    }
}
