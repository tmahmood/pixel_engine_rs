use piston::{RenderArgs, UpdateArgs};
use crate::game_engine::game_board::PixelMap;

pub trait GameEvents {
    fn render(&mut self, args: &RenderArgs) -> PixelMap;
    fn update(&mut self, args: &UpdateArgs);
}
