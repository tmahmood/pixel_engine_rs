extern crate snake_game;
use snake_game::{game_engine, block_app};
use snake_game::game_engine::{GameEngine};
use snake_game::block_app::App;
use opengl_graphics::{GlGraphics, OpenGL};

fn main() {
    let opengl = OpenGL::V4_5;
    let mut ge = GameEngine::new(
        [
            block_app::WINDOW_WIDTH as f64,
            block_app::WINDOW_HEIGHT as f64
        ], opengl
    );
    let mut app = App::new(GlGraphics::new(opengl));
    ge.game_loop(&mut app);
}
