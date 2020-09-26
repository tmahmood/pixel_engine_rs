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
    pub x: f32,
    pub y: f32,
    pub color: [f32; 4],
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
                x: 0.0, y: 0.0,
                color: [0.0, 0.0, 0.0, 1.0],
                shape: ShapeKind::None, index: 0,
            }, index: None
        }
    }

    pub fn new(s: ShapeKind, x: f32, y: f32) -> Self {
        BlockBuilder {
            block: Block {
                x, y,
                color: [1.0, 1.0, 1.0, 1.0],
                shape: s, index: 0,
            }, index: None
        }
    }

    pub fn rect(x: f32, y: f32) -> Self {
        BlockBuilder::new(ShapeKind::Rect, x, y)
    }

    pub fn line(x: f32, y: f32) -> Self {
        BlockBuilder::new(ShapeKind::Line, x, y)
    }

    pub fn circle(x: f32, y: f32) -> Self {
        BlockBuilder::new(ShapeKind::Circle, x, y)
    }

    pub fn ellipse(x: f32, y: f32) -> Self {
        BlockBuilder::new(ShapeKind::Ellipse, x, y)
    }

    pub fn polygon(x: f32, y: f32) -> Self {
        BlockBuilder::new(ShapeKind::Polygon, x, y)
    }

    pub fn color(&mut self, color: Vec<f32>) -> &mut Self {
        self.block.color = [color[0], color[1], color[2], color[3]];
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

    pub fn get_shape_info(&self, point_list: &Vec<Vec<f32>>) -> Vec<f32> {
        point_list[self.index].to_vec()
    }

    pub fn get_shape_kind(&self) -> ShapeKind {
        self.shape.to_owned()
    }

    pub fn get_index(&self) -> usize {
        self.index
    }
}

