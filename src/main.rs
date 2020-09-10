#![feature(vec_remove_item)]
extern crate gd_learn_001;
extern crate uuid;
extern crate image;

pub mod ray_tracing;
pub mod block_app;

use crate::block_app::BlockGame;
use glutin_window::OpenGL;
use gd_learn_001::game_engine::{GameEngineData, init_game_engine, game_loop};
use gd_learn_001::game_engine::game_data_model::GameDataModel;

fn main_piston() {
    let opengl = OpenGL::V4_5;
    let mut app = BlockGame::new();
    let mut ge = init_game_engine(
        [
            app.get_window_width() as f64,
            app.get_window_height() as f64
        ], opengl,
    );
    game_loop(app, ge);
}

fn main() {
    main_piston();
}
