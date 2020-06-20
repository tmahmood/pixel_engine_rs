extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub mod point3d;
pub mod game_app;
pub mod shapes;
pub mod commands;

pub mod game_engine {

    use piston_window::PistonWindow as Window;
    use opengl_graphics::{GlGraphics, OpenGL};
    use piston::event_loop::{EventSettings, Events};
    use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
    use piston::window::WindowSettings;
    use graphics::math::Vec2d;
    use graphics::{math, clear, Context};
    use crate::game_app::GameApp;
    use crate::{BLACK, GREEN, RED};

    use graphics::types::Rectangle;
    use graphics::rectangle::rectangle_by_corners;
    use crate::shapes::ShapeKind;

    fn draw_rect(rect_base: [f64;4], shape: [f64;2], c: &Context, gl: &mut GlGraphics, block: [f64;2]) {
        use graphics::*;
        rectangle(RED, rect_base,
                  // translate to actual position in the viewport
                  c.transform.trans(
                      shape[0] * block[0],
                      shape[1] * block[1]
                  ), gl);
    }

    pub struct GameEngine {
        pub size: [f64; 2],
        pub window: Window,
        pub gl: GlGraphics,
    }

    impl GameEngine {

        pub fn new(size: [f64;2], opengl: OpenGL) -> Self {
            let size: [f64; 2] = size;
            let window = WindowSettings::new("Game", size)
                .graphics_api(opengl)
                .exit_on_esc(true)
                .build()
                .unwrap();
            let gl = GlGraphics::new(opengl);
            GameEngine { gl, size, window }
        }

        pub fn game_loop<T: GameApp>(&mut self, app:&mut T) {
            use graphics::*;
            let mut events = Events::new(EventSettings::new());
            let rect_base = [0.0, 0.0, app.get_block_width(), app.get_block_height()];
            while let Some(e) = events.next(&mut self.window) {
                if let Some(args) = e.render_args() {
                    // get all the drawable
                    let shapes = app.get_drawables(&args);
                    // doing drawing stuffS
                    self.gl.draw(args.viewport(), |c, gl| {
                        // clear the screen
                        clear(BLACK, gl);
                        // draw things, we check what kind of shape we are working with
                        // and draw appropriate shape
                        for shape in shapes {
                            match shape.1 {
                                ShapeKind::Rect => {
                                    draw_rect(
                                        rect_base,
                                        [shape.0[0], shape.0[1]], &c, gl,
                                        [
                                            app.get_block_width(),
                                            app.get_block_height()
                                        ]
                                    );
                                },
                                ShapeKind::Circle => {},
                                ShapeKind::Triangle => {},
                                ShapeKind::Line => {},
                                ShapeKind::Pixel => {},
                            };
                        }
                    });
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
