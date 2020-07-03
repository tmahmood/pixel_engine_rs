extern crate gd_learn_001;
extern crate uuid;
extern crate image;

pub mod ray_tracing;
pub mod block_app;

use gd_learn_001::{game_engine};
use gd_learn_001::game_engine::{GameEngine};
use opengl_graphics::{OpenGL};
use crate::block_app::BlockGame;
use piston_window::TextureContext;


fn main() {
    let opengl = OpenGL::V4_5;
    let mut ge = GameEngine::new(
        [
            block_app::WINDOW_WIDTH as f64,
            block_app::WINDOW_HEIGHT as f64
        ], opengl,
    );
    let mut app: BlockGame = BlockGame::new();
    ge.game_loop(&mut app);
}
