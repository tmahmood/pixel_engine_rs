extern crate snake_game;
use snake_game::{game_engine};
use snake_game::game_engine::{GameEngine};
use opengl_graphics::{OpenGL};
use crate::block_app::App;

pub mod block_app;



fn main() {
    let opengl = OpenGL::V4_5;
    let mut ge = GameEngine::new(
        [
            block_app::WINDOW_WIDTH as f64,
            block_app::WINDOW_HEIGHT as f64
        ], opengl
    );
    let mut app = App::new();
    ge.game_loop(&mut app);
}
