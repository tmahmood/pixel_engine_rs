extern crate gd_learn_001;
extern crate uuid;
extern crate image;

pub mod ray_tracing;
pub mod block_app;

use crate::block_app::BlockGame;
use ggez::{event, ContextBuilder};
use ggez::graphics::Canvas;


fn main() {
    let (mut context, mut events_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    let canvas = Canvas::with_window_size(&mut context);
    let mut app: BlockGame = BlockGame::new(canvas.unwrap());
    event::run(&mut context, &mut events_loop, &mut app);
}
