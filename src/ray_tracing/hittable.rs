use crate::ray_tracing::ray::Ray;
use gd_learn_001::game_engine::point3d::{Point3D, dot};

pub struct HitRecord {
    pub p: Point3D,
    pub normal: Point3D,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Point3D) {
        self.front_face = dot(&r.get_direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal.clone() } else { -outward_normal.clone() };
    }

}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}


