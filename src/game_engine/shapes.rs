use piston::window::Position;
use std::collections::HashMap;
use piston::input::UpdateArgs;

#[derive(Copy, Clone)]
pub enum ShapeKind {
    Rect,
    Polygon,
    Line,
    Pixel,
    Ellipse,
}

#[derive(Copy, Clone)]
pub struct Block {
    pub color: [f32; 4],
    pub dx: f32,
    pub dy: f32,
    pub shape: ShapeKind,
    pub index: usize,
}

impl Block {
    pub fn new(points: Vec<f64>, color: [f32; 4], dx: f32, dy: f32, shape: ShapeKind, point_list: &mut Vec<Vec<f64>>) -> Self {
        let i = point_list.len();
        point_list.push(points);
        Block {
            color,
            dx,
            dy,
            shape,
            index: i,
        }
    }

    pub fn update_position(&mut self, args: &UpdateArgs, points: &mut Vec<f64>) {
        match self.shape {
            ShapeKind::Rect => {
                points[0] = points[0] + (self.dx * args.dt as f32) as f64;
                points[1] = points[1] + (self.dy * args.dt as f32) as f64;
            },
            ShapeKind::Polygon => {
                points.chunks_mut(2).for_each(|item|{
                    item[0] = item[0] + (self.dx * args.dt as f32) as f64;
                    item[1] = item[1] + (self.dy * args.dt as f32) as f64;
                });
            },
            ShapeKind::Line => {},
            ShapeKind::Pixel => {},
            ShapeKind::Ellipse => {
                points[0] = points[0] + (self.dx * args.dt as f32) as f64;
                points[1] = points[1] + (self.dy * args.dt as f32) as f64;
            },
        }
    }

    pub fn get_shape_info(&self, point_list: &Vec<Vec<f64>>) -> Vec<f64> {
        point_list[self.index].to_vec()
    }

    pub fn get_shape_kind(&self) -> ShapeKind {
        self.shape.to_owned()
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_movement_rate(&self) -> [f32; 2] {
        [self.dx, self.dy]
    }
}

