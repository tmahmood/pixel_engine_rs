use piston::{RenderArgs, UpdateArgs, Event, Button, Key, MouseButton, ControllerButton, HatState, ControllerHat};
use crate::game_engine::game_board::PixelMap;

pub trait PistonGameEvents {
    fn update_game_board(&mut self, args: &RenderArgs) -> PixelMap;
    fn update(&mut self, args: &UpdateArgs);
    fn handle_press_events(&mut self, button: &Button);
    fn handle_release_events(&mut self, button: &Button);
    // events
}
