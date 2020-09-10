use gd_learn_001::{BLACK, GREEN, RED};
use std::collections::HashMap;
use gd_learn_001::game_engine::shapes::{Block, ShapeKind, BlockBuilder, EllipseBuilder, RectBuilder, LineBuilder, CircleBuilder, PolygonBuilder};
use gd_learn_001::game_engine::game_data_model::GameDataModel;
use gd_learn_001::game_engine::parse_block_list::parse_block_list;
use std::fs;
use gd_learn_001::game_engine::game_board::{GameBoard, PixelMap, Pixel};
use gd_learn_001::game_engine::{draw_shapes};
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use gd_learn_001::game_engine::game_events::PistonGameEvents;
use piston::{UpdateArgs, RenderArgs, Event, PressEvent, ReleaseEvent, Key, MouseButton};
use piston::Button;


// most of these configurations can be loaded from config file later
pub const WINDOW_WIDTH: i32 = 800;
pub const WINDOW_HEIGHT: i32 = 800;
pub const MAP_SIZE: i32 = 200;
pub const BOARD_WIDTH: f32 = 800.0;
pub const BOARD_HEIGHT: f32 = 800.0;

pub mod piston_game_events;

pub struct ControlButtons {
    block_index: usize,
    controls: Vec<Key>,
}

pub struct BlockGame {
    pub blocks: Vec<MovingObjects>,
    pub point_list: Vec<Vec<f32>>,
    pub key_pressed: Vec<Key>,
    pub control: Vec<ControlButtons>,
}

impl BlockGame {
    pub fn new() -> Self {
        let mut point_list = vec![];
        BlockGame {
            blocks: vec![
                // MovingObjects {
                //     dx: 8.0,
                //     dy: 8.0,
                //     angle: 0.0,
                //     max_speed: 5.0,
                //     block: BlockBuilder::rect()
                //         .points(RectBuilder::position(23.0, 15.0)
                //                     .size(20.0, 20.0).build(), &mut point_list)
                //         .color(Vec::from(RED))
                //         .build(),
                // },
                // MovingObjects {
                //     dx: 9.0,
                //     dy: 3.0,
                //     angle: 0.0,
                //     max_speed: 5.0,
                //     block: BlockBuilder::ellipse()
                //         .points(EllipseBuilder::position(23.0, 15.0).rad(10.0, 10.0).build(),
                //                 &mut point_list)
                //         .color(Vec::from(RED))
                //         .build(),
                // },
                // MovingObjects {
                //     dx: 40.0,
                //     dy: 10.0,
                //     angle: 0.0,
                //     max_speed: 9.0,
                //     block: BlockBuilder::circle()
                //         .color(vec![43.0, 87.0, 23.0, 100.0])
                //         .points(CircleBuilder::new(56.0, 70.0, 10.0).build(), &mut point_list)
                //         .build(),
                // },
                // MovingObjects {
                //     dx: 15.0,
                //     dy: 10.0,
                //     angle: 0.0,
                //     max_speed: 0.0,
                //     block: BlockBuilder::polygon()
                //         .color(vec![43.0, 87.0, 23.0, 100.0])
                //         .points(
                //             PolygonBuilder::start_point(10.0, 10.0)
                //                 .add_point(15.0, 15.0)
                //                 .add_point(25.0, 25.0)
                //                 .build(), &mut point_list
                //         )
                //         .build(),
                // },
                MovingObjects {
                    dx: 10.0,
                    dy: 3.0,
                    angle: 0.0,
                    max_speed: 6.0,
                    block: BlockBuilder::line()
                        .points(LineBuilder::start_point(40.0, 40.0).end_point(55.0, 58.0).build(),
                                &mut point_list)
                        .color(Vec::from(GREEN))
                        .build(),
                },
            ],
            point_list,
            key_pressed: Vec::new(),
            control: vec![],
        }
    }
}

impl GameDataModel for BlockGame {
    fn get_drawables(&self) -> (Vec<Block>, &Vec<Vec<f32>>) {
        let block_list = self.blocks.iter().map(|b| b.block).collect();
        (block_list, &self.point_list)
    }
    fn get_window_width(&self) -> i32 {
        WINDOW_WIDTH
    }
    fn get_window_height(&self) -> i32 {
        WINDOW_HEIGHT
    }
    fn get_map_size(&self) -> i32 {
        MAP_SIZE
    }
    fn get_board_width(&self) -> f32 {
        BOARD_WIDTH
    }
    fn get_board_height(&self) -> f32 {
        BOARD_HEIGHT
    }
    fn get_block_width(&self) -> f32 {
        BOARD_WIDTH / MAP_SIZE as f32
    }
    fn get_block_height(&self) -> f32 {
        BOARD_HEIGHT / MAP_SIZE as f32
    }
}

pub fn wrap_points(ix: f32, iy: f32, map_size: f32) -> (f32, f32) {
    let mut ox = ix;
    let mut oy = iy;
    if ix < 0.0 { ox = ix + map_size } else if ix >= map_size { ox = ix - map_size; }
    if iy < 0.0 { oy = iy + map_size } else if iy >= map_size { oy = iy - map_size; }
    (ox, oy)
}

pub struct MovingObjects {
    pub dx: f32,
    pub dy: f32,
    pub angle: f32,
    pub max_speed: f32,
    pub block: Block,
}

pub fn update_position(shape: ShapeKind, dt: f64, points: &mut Vec<f32>, dx: f32, dy: f32, bw: f32, bh: f32) {
    match shape {
        ShapeKind::Rect => {
            points[0] = points[0] + (dx * dt as f32) as f32;
            points[1] = points[1] + (dy * dt as f32) as f32;
            if points[0] >= bw { points[0] = 0.0 }
            if points[1] >= bh { points[1] = 0.0 }
        }
        ShapeKind::Polygon => {
            points.chunks_mut(2).for_each(|item| {
                item[0] = item[0] + (dx * dt as f32) as f32;
                item[1] = item[1] + (dy * dt as f32) as f32;
                if item[0] >= bw { item[0] = 0.0 }
                if item[1] >= bh { item[1] = 0.0 }
            });
        }
        ShapeKind::Line => {
            points[0] = points[0] + (dx * dt as f32) as f32;
            points[1] = points[1] + (dy * dt as f32) as f32;
            points[2] = points[2] + (dx * dt as f32) as f32;
            points[3] = points[3] + (dy * dt as f32) as f32;
            if points[0] >= bw {
                points[2] = (points[2] - points[0]).abs();
                points[0] = 0.0;
            }
            if points[1] >= bw {
                points[3] = (points[3] - points[1]).abs();
                points[1] = 0.0;
            }
        }
        ShapeKind::Ellipse => {
            points[0] = points[0] + (dx * dt as f32) as f32;
            points[1] = points[1] + (dy * dt as f32) as f32;
            if points[0] >= bw { points[0] = 0.0 }
            if points[1] >= bh { points[1] = 0.0 }
        }
        ShapeKind::Circle => {
            points[0] = points[0] + (dx * dt as f32) as f32;
            points[1] = points[1] + (dy * dt as f32) as f32;
            if points[0] >= bw { points[0] = 0.0 }
            if points[1] >= bh { points[1] = 0.0 }
        }
        ShapeKind::None => {
            panic!("This should not happen")
        }
    }
}
