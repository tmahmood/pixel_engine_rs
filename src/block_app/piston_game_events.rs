use gd_learn_001::game_engine::game_events::PistonGameEvents;
use crate::block_app::{BlockGame, wrap_points, update_position};
use piston::{RenderArgs, UpdateArgs, Key, Event, ControllerButton, ControllerHat, MouseButton, Button};
use gd_learn_001::game_engine::game_board::PixelMap;
use gd_learn_001::game_engine::game_data_model::GameDataModel;
use gd_learn_001::game_engine::draw_shapes;
use rayon::prelude::*;

impl PistonGameEvents for BlockGame {
    // do render things
    fn render(&mut self, args: &RenderArgs) -> PixelMap {
        let map_size = self.get_map_size();
        let mut b = Vec::new();
        let mut p = Vec::new();
        {
            let (_b, _p) = self.get_drawables();
            b = _b.clone();
            p = _p.clone();
        }
        let mut pixels = draw_shapes(&b, &p);
        pixels.values_mut().par_bridge().for_each(|value| {
            let (ox, oy) = wrap_points(value.point.x, value.point.y, map_size as f32);
            value.point.x = ox;
            value.point.y = oy;
        });
        pixels
    }

    fn update(&mut self, args: &UpdateArgs) {
        let dw = self.get_map_size() as f32;
        let dh = self.get_map_size() as f32;
        for obj in &mut self.blocks {
            if self.key_pressed.contains(&Key::A) {
                obj.angle -= 5.0 * args.dt as f32;
            }
            if self.key_pressed.contains(&Key::D) {
                obj.angle += 5.0 * args.dt as f32;
            }
            if self.key_pressed.contains(&Key::W) {
                obj.dx += obj.angle.sin();
                obj.dy += -obj.angle.cos();
            }
            if self.key_pressed.contains(&Key::S) {
                obj.dx -= obj.angle.sin() * args.dt as f32;
                obj.dy -= -obj.angle.cos() * args.dt as f32;
            }
            update_position(
                obj.block.shape, args.dt,
                &mut self.point_list[obj.block.index],
                obj.dx, obj.dy, dw, dh,
            );
        }
    }

    fn handle_press_events(&mut self, button: &Button) {
        match button {
            Button::Keyboard(k) => {
                self.key_pressed.push(k.clone());
            }
            (_) => {}
        }
    }

    fn handle_release_events(&mut self, button: &Button) {
        //unimplemented!()
    }
}
