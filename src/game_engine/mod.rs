pub mod point3d;
pub mod game_app;
pub mod shapes;
pub mod commands;

use piston_window::PistonWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::math::Vec2d;
use graphics::{math, clear, Context};
use crate::{BLACK, GREEN, RED};

use graphics::types::Rectangle;
use graphics::rectangle::rectangle_by_corners;
use crate::game_engine::shapes::ShapeKind;
use crate::game_engine::game_app::GameApp;

pub struct GameEngine {
    pub size: [f64; 2],
    pub window: Window,
    pub gl: GlGraphics,
}

impl GameEngine {
    pub fn new(size: [f64; 2], opengl: OpenGL) -> Self {
        let size: [f64; 2] = size;
        let window = WindowSettings::new("Game", size)
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
        let gl = GlGraphics::new(opengl);
        GameEngine { gl, size, window }
    }


    pub fn game_loop<T: GameApp>(&mut self, app: &mut T) {
        use graphics::*;
        let mut events = Events::new(EventSettings::new());
        let rect_base = [0.0, 0.0, app.get_block_width(), app.get_block_height()];
        while let Some(e) = events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                // get all the drawable
                let (shapes, point_list) = app.get_drawables(&args);
                let block_width = app.get_block_width();
                let block_height = app.get_block_height();
                // doing drawing stuffS
                self.gl.draw(args.viewport(), |c, gl| {
                    // clear the screen
                    clear(BLACK, gl);
                    // draw things, we check what kind of shape we are working with
                    // and draw appropriate shape
                    for block in shapes {
                        let k = &point_list[block.index];
                        match block.shape {
                            ShapeKind::Rect => {
                                let t = c.transform.trans(k[0] * block_width, k[1] * block_height);
                                rectangle(block.color, rect_base, t, gl);
                            }
                            ShapeKind::Ellipse => {
                                let t = c.transform.trans(k[0] * block_width, k[1] * block_height);
                                ellipse(block.color, rect_base, t, gl);
                            }
                            ShapeKind::Polygon => {
                                let p:Vec<[f64;2]> = point_list[block.index]
                                    .chunks(2)
                                    .map(|item|
                                        [item[0] * block_width, item[1] * block_height]
                                    ).collect();
                                polygon(block.color, &p, c.transform, gl);
                            }
                            ShapeKind::Line => {
                                let line_points: &Vec<f64> = &point_list[block.index];
                                line(block.color, line_points[4], [
                                    line_points[0] * block_width, line_points[1] * block_height,
                                    line_points[2] * block_width, line_points[3] * block_height
                                ], c.transform, gl);
                            }
                            ShapeKind::Pixel => {}
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
