use std::ops;
use std::ops::{Neg, Mul, Index};


#[derive(Copy, Clone)]
pub struct Point3D {
    e: [f32; 3]
}

impl Point3D {

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point3D { e: [x, y, z] }
    }


    pub fn x(&self) -> f32 { self.e[0] }
    pub fn y(&self) -> f32 { self.e[1] }
    pub fn z(&self) -> f32 { self.e[2] }

    pub fn x_i32(&self) -> i32 { self.e[0] as i32}
    pub fn y_i32(&self) -> i32 { self.e[1] as i32}
    pub fn z_i32(&self) -> i32 { self.e[2] as i32}

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn display(&self) {
        println!("{} {} {}", self.x(), self.y(), self.z());
    }

    pub fn dot(&self, v2: Point3D) -> f32 {
        self.x() * v2.x() + self.y() * v2.y() + self.z() * v2.z()
    }

    pub fn unit_vector(&self) -> Point3D {
        *self / self.length()
    }

    pub fn cross(&self, v: Point3D) -> Point3D {
        Point3D {
            e: [
                self.y() * v.z() - self.z() * v.y(),
                self.z() * v.x() - self.x() * v.z(),
                self.x() * v.y() - self.y() * v.x(),
            ]
        }
    }
}

impl ops::Neg for Point3D {
    type Output = Point3D;
    fn neg(self) -> Self::Output {
        self.mul(-1.0) } }

impl ops::Add<Point3D> for Point3D {
    type Output = Point3D;
    fn add(self, _rhs: Point3D) -> Self::Output {
        Point3D {
            e: [
                self.e[0] + _rhs.e[0],
                self.e[1] + _rhs.e[1],
                self.e[2] + _rhs.e[2],
            ]
        }
    }
}

impl ops::AddAssign<Point3D> for Point3D {
    fn add_assign(&mut self, _rhs: Point3D) {
        self.e[0] += _rhs.e[0];
        self.e[1] += _rhs.e[1];
        self.e[2] += _rhs.e[2];
    }
}

impl ops::Sub<Point3D> for Point3D {
    type Output = Point3D;
    fn sub(self, _rhs: Point3D) -> Self::Output {
        Point3D {
            e: [
                self.e[0] - _rhs.e[0],
                self.e[1] - _rhs.e[1],
                self.e[2] - _rhs.e[2],
            ]
        }
    }
}

impl ops::Mul<Point3D> for Point3D {
    type Output = Point3D;
    fn mul(self, rhs: Point3D) -> Self::Output {
        Point3D {
            e: [
                self.e[0] * rhs.x(),
                self.e[1] * rhs.y(),
                self.e[2] * rhs.z(),
            ]
        }
    }
}

impl ops::Mul<f32> for Point3D {
    type Output = Point3D;
    fn mul(self, rhs: f32) -> Self::Output {
        Point3D {
            e: [
                self.e[0] * rhs,
                self.e[1] * rhs,
                self.e[2] * rhs,
            ]
        }
    }
}

impl ops::MulAssign<f32> for Point3D {
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] = self.x() * rhs;
        self.e[1] = self.y() * rhs;
        self.e[2] = self.z() * rhs;
    }
}

impl ops::Div<f32> for Point3D {
    type Output = Point3D;

    fn div(self, rhs: f32) -> Point3D {
        self * (1f32 / rhs)
    }
}

impl ops::DivAssign<f32> for Point3D {
    fn div_assign(&mut self, rhs: f32) {
        self.e[0] = self.x() / rhs;
        self.e[1] = self.y() / rhs;
        self.e[2] = self.z() / rhs;
    }
}

#[cfg(test)]
mod tests {
    use crate::game_engine::point3d::Point3D;

    #[test]
    fn test_cross() {
        let v1 = Point3D { e: [2f32, 3f32, 4f32] };
        let v2 = Point3D { e: [5f32, 6f32, 7f32] };
        let v3 = v1.cross(v2);
        assert_eq!(v3.e, [-3.0, 6.0, -3.0]);
    }

    #[test]
    fn test_div_assign() {
        let mut v = Point3D { e: [6f32, 9f32, 12f32] };
        v /= 3f32;
        assert_eq!(v.e, [2f32, 3f32, 4f32]);
    }


    #[test]
    fn test_div() {
        let v = Point3D { e: [6f32, 9f32, 12f32] };
        let k = v / 3f32;
        assert_eq!(k.e, [2f32, 3f32, 4f32])
    }

    #[test]
    fn test_mul_assign() {
        let mut v1 = Point3D { e: [5f32, 2f32, 5f32] };
        v1 *= 3f32;
        assert_eq!(v1.e, [15f32, 6f32, 15f32])
    }


    #[test]
    fn test_sub() {
        let v = Point3D { e: [5f32, 2f32, 5f32] } - Point3D { e: [4f32, 1f32, 5f32] };
        assert_eq!(v.e, [1f32, 1f32, 0f32])
    }

    #[test]
    fn test_neg() {
        let v = -Point3D { e: [5f32, 2f32, 5f32] };
        assert_eq!(v.e, [-5f32, -2f32, -5f32])
    }


    #[test]
    fn test_addition_assign() {
        let mut v = Point3D { e: [3f32, 2f32, 5f32] };
        v += Point3D { e: [4f32, 1f32, 5f32] };
        assert_eq!(v.e, [7f32, 3f32, 10f32])
    }
}

pub fn dot(a: &Point3D, b: &Point3D) -> f32 {
    a.x() * b.x() + a.y() * b.y() + a.z() * b.z()
}

pub fn unit_vector(a: &Point3D) -> Point3D {
    *a / a.length()
}

pub fn cross(a: &Point3D, v: &Point3D) -> Point3D {
    Point3D {
        e: [
            a.y() * v.z() - a.z() * v.y(),
            a.z() * v.x() - a.x() * v.z(),
            a.x() * v.y() - a.y() * v.x(),
        ]
    }
}

