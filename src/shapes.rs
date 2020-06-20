use piston::window::Position;

pub enum ShapeKind {
    Rect,
    Circle,
    Triangle,
    Line,
    Pixel,
}

pub trait Shape {
    fn get_shape_kind(&self) -> ShapeKind;
    fn get_shape_info(&self) -> Vec<f64>;
    fn get_movement_rate(&self) -> [f32; 2];

    fn set_shape_position(&mut self, x: f64, y: f64);
}

#[derive(Copy, Clone)]
pub struct Block {
    position: [f64; 2],
    color: [f32; 4],
    dx: f32,
    dy: f32,
}

impl Block {
    pub fn new(position: [f64; 2], color: [f32; 4], dx: f32, dy: f32) -> Self {
        Block {
            position,
            color,
            dx,
            dy
        }
    }
}

impl Shape for Block {
    fn get_shape_kind(&self) -> ShapeKind {
        ShapeKind::Rect
    }

    fn get_shape_info(&self) -> Vec<f64> {
        Vec::from(self.position)
    }

    fn get_movement_rate(&self) -> [f32; 2] {
        [self.dx, self.dy]
    }

    fn set_shape_position(&mut self, x: f64, y: f64) {
        self.position = [x, y];
    }
}

