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
                shape: ShapeKind::None, index: 0,
            }, index: None
        }
    }

    pub fn new(s: ShapeKind) -> Self {
        BlockBuilder {
            block: Block {
                color: [1.0, 1.0, 1.0, 1.0],
                shape: s, index: 0,
            }, index: None
        }
    }

    pub fn line() -> Self {
        BlockBuilder::new(ShapeKind::Line)
    }

    pub fn circle() -> Self {
        BlockBuilder::new(ShapeKind::Circle)
    }

    pub fn rect() -> Self {
        BlockBuilder::new(ShapeKind::Rect)
    }

    pub fn ellipse() -> Self {
        BlockBuilder::new(ShapeKind::Ellipse)
    }

    pub fn polygon() -> Self {
        BlockBuilder::new(ShapeKind::Polygon)
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

pub struct LineBuilder {
    points: [f32; 4]
}

impl LineBuilder {
    pub fn start_point(x: f32, y:f32) -> Self {
        LineBuilder {
            points: [x, y, 0.0, 0.0]
        }
    }

    pub fn to(&mut self, px: f32, py: f32) -> &mut Self {
        self.points[2] = self.points[0] + px;
        self.points[3] = self.points[1] + py;
        self
    }

    pub fn end_point(&mut self, x: f32, y: f32) -> &mut Self {
        self.points[2] = x;
        self.points[3] = y;
        self
    }
    pub fn build(&mut self) -> Vec<f32> {
        Vec::from(self.points)
    }
}
pub struct RectBuilder {
    points: [f32; 4]
}

impl RectBuilder {
    pub fn position(x: f32, y:f32) -> Self {
        RectBuilder {
            points: [x, y, 0.0, 0.0]
        }
    }

    pub fn size(&mut self, w: f32, h: f32) -> &mut Self {
        self.points[2] = w;
        self.points[3] = h;
        self
    }

    pub fn build(&mut self) -> Vec<f32> {
        Vec::from(self.points)
    }
}

pub struct EllipseBuilder {
    points: [f32; 4]
}

impl EllipseBuilder {

    pub fn position(x: f32, y: f32) -> Self {
        EllipseBuilder  {
            points: [x, y, 0.0, 0.0]
        }
    }

    pub fn rad(&mut self, a: f32, b: f32) -> &mut Self {
        self.points[2] = a;
        self.points[3] = b;
        self
    }

    pub fn build(&mut self) -> Vec<f32> {
        Vec::from(self.points)
    }
}

pub struct CircleBuilder {
    points: [f32; 3]
}

impl CircleBuilder {
    pub fn new(x: f32, y: f32, r: f32) -> Self {
        CircleBuilder {
            points: [x, y, r]
        }
    }

    pub fn build(&self) -> Vec<f32> {
        Vec::from(self.points)
    }
}

pub struct PolygonBuilder {
    points: Vec<f32>,
}

impl PolygonBuilder {
    pub fn start_point(x: f32, y: f32) -> Self {
        PolygonBuilder { points: vec![x, y] }
    }

    pub fn add_point(&mut self, x: f32, y: f32) -> &mut Self {
        self.points.extend_from_slice(&[x, y]);
        self
    }

    pub fn build(&mut self) -> Vec<f32> {
        self.points.extend_from_slice(&[self.points[0], self.points[1]]);
        self.points.clone()
    }
}

