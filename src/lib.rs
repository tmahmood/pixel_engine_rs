extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate gfx;

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub mod game_engine;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
