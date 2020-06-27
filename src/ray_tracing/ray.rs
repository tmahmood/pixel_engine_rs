use snake_game::game_engine::point3d::Point3D;

pub struct Ray {
    origin: Point3D,
    direction: Point3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Point3D) -> Self {
        Ray {
            origin,
            direction
        }
    }

    pub fn get_origin(&self) -> Point3D {
        self.origin
    }

    pub fn get_direction(&self) -> Point3D {
        self.direction
    }

    pub fn at(&self, t: f32) -> Point3D {
        self.origin + self.direction * t
    }
}

