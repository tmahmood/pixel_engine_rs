use std::collections::HashMap;
use crate::BLACK;
use std::convert::TryInto;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum ShapeKind {
    Rect,
    Polygon,
    Line,
    Ellipse,
    Circle,
    None
}

#[derive(Copy, Clone, Debug)]
pub struct Block {
    pub color: [f32; 4],
    pub dx: f32,
    pub dy: f32,
    pub shape: ShapeKind,
    pub index: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct BlockBuilder {
    block: Block,
    index: Option<usize>,
}

impl BlockBuilder {
    pub fn empty() -> Self {
        BlockBuilder {
            block: Block {
                color: [0.0, 0.0, 0.0, 1.0],
                dx: 0.0, dy: 0.0,
                shape: ShapeKind::None, index: 0,
            }, index: None
        }
    }
    pub fn new(s: ShapeKind) -> Self {
        BlockBuilder {
            block: Block {
                color: [1.0, 1.0, 1.0, 1.0],
                dx: 0.0, dy: 0.0,
                shape: s, index: 0,
            }, index: None
        }
    }

    pub fn from_str(s: String) -> Self {
        if s.starts_with("R") {
            BlockBuilder::new(ShapeKind::Rect)
        } else if s.starts_with("E") {
            BlockBuilder::new(ShapeKind::Ellipse)
        } else if s.starts_with("L") {
            BlockBuilder::new(ShapeKind::Line)
        } else if s.starts_with("P") {
            BlockBuilder::new(ShapeKind::Polygon)
        } else if s.starts_with("C") {
            BlockBuilder::new(ShapeKind::Circle)
        } else {
            panic!("Invalid shapes")
        }
    }

    pub fn color(&mut self, color: Vec<f32>) -> &mut Self {
        self.block.color = [color[0], color[1], color[2], color[3]];
        self
    }

    pub fn velocity(&mut self, dt: Vec<f32>) -> &mut Self {
        self.block.dx = dt[0];
        self.block.dy = dt[1];
        self
    }

    pub fn points(&mut self, points: Vec<f32>, points_list: &mut Vec<Vec<f32>>) -> &mut Self {
        self.index = Some(points_list.len());
        points_list.push(points);
        self
    }

    pub fn build(&mut self) -> Block {
        if self.block.shape == ShapeKind::None {
            panic!("Block is not initialized");
        }
        if self.index.is_none() {
            panic!("Position information not initialized");
        }
        self.block.index = self.index.unwrap();
        return self.block;
    }
}

impl Block {

    pub fn update_position(&mut self, dt: f64, points: &mut Vec<f32>) {
        match self.shape {
            ShapeKind::Rect => {
                points[0] = points[0] + (self.dx * dt as f32) as f32;
                points[1] = points[1] + (self.dy * dt as f32) as f32;
            }
            ShapeKind::Polygon => {
                points.chunks_mut(2).for_each(|item| {
                    item[0] = item[0] + (self.dx * dt as f32) as f32;
                    item[1] = item[1] + (self.dy * dt as f32) as f32;
                });
            }
            ShapeKind::Line => {
                points[0] = points[0] + (self.dx * dt as f32) as f32;
                points[1] = points[1] + (self.dy * dt as f32) as f32;
                points[2] = points[2] + (self.dx * dt as f32) as f32;
                points[3] = points[3] + (self.dy * dt as f32) as f32;
            }
            ShapeKind::Ellipse => {
                points[0] = points[0] + (self.dx * dt as f32) as f32;
                points[1] = points[1] + (self.dy * dt as f32) as f32;
            }
            ShapeKind::Circle => {
                points[0] = points[0] + (self.dx * dt as f32) as f32;
                points[1] = points[1] + (self.dy * dt as f32) as f32;
            }
            ShapeKind::None => {
                panic!("This should not happen")
            }
        }
    }

    pub fn get_shape_info(&self, point_list: &Vec<Vec<f32>>) -> Vec<f32> {
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

