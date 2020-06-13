extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub mod block_app;

pub mod game_app {
    use piston::input::{RenderArgs, UpdateArgs};

    pub trait GameApp {
        fn render(&mut self, args: &RenderArgs);
        fn update(&mut self, args: &UpdateArgs);
    }

}

pub mod game_engine {

    use glutin_window::GlutinWindow as Window;
    use opengl_graphics::{GlGraphics, OpenGL};
    use piston::event_loop::{EventSettings, Events};
    use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
    use piston::window::WindowSettings;
    use graphics::math::Vec2d;
    use graphics::math;
    use crate::game_app::GameApp;

    pub mod commands;

    pub struct GameEngine {
        pub size: [f64; 2],
        pub window: Window,
    }

    impl GameEngine {

        pub fn new(size: [f64;2], opengl: OpenGL) -> Self {
            // Change this to OpenGL::V2_1 if not working.
            let size: [f64; 2] = size;
            GameEngine {
                size,
                window: WindowSettings::new("Game", size)
                    .graphics_api(opengl)
                    .exit_on_esc(true)
                    .build()
                    .unwrap()
            }
        }

        pub fn game_loop<T: GameApp>(&mut self, app:&mut T) {
            let mut events = Events::new(EventSettings::new());
            while let Some(e) = events.next(&mut self.window) {
                if let Some(args) = e.render_args() {
                    app.render(&args);
                }
                if let Some(args) = e.update_args() {
                    app.update(&args);
                }
            }

        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
