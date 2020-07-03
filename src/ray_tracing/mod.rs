use gd_learn_001::game_engine::point3d::{Point3D, dot};
use crate::ray_tracing::ray::Ray;
use piston_window::{TextureContext, G2dTexture, TextureSettings, Texture};

const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 384;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;

pub mod ray;
pub mod hittable;

pub mod sphere {
    use gd_learn_001::game_engine::point3d::{Point3D, dot};
    use crate::ray_tracing::hittable::{Hittable, HitRecord};
    use crate::ray_tracing::ray::Ray;

    struct Sphere {
        center: Point3D,
        radius: f32,
    }

    impl Hittable for Sphere {
        fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
            let oc = r.get_origin() - self.center;
            let dir = r.get_direction();
            let a = dir.length_squared();
            let c = oc.length_squared() - self.radius * self.radius;
            let half_b = dot(&oc, &dir);
            let discriminant = half_b * half_b - a * c;

            if discriminant > 0.0 {
                let root = discriminant.sqrt();
                let t1 = (-half_b - root) / a;
                let t2 = (-half_b + root) / a;
                let mut temp = 0.0;
                if t1 < t_max && t1 > t_min {
                    temp = t1;
                }
                if t2 < t_max && t2 > t_min {
                    temp = t2;
                }
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, &outward_normal);
                return true;
            }
            return false;
        }
    }
}


fn ray_color(r: Ray) -> Point3D {
    let t = hit_sphere(Point3D::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = (r.at(t) - Point3D::new(0.0, 0.0, -1.0)).unit_vector();
        return Point3D::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }
    let unit_direction = r.get_direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Point3D::new(1.0, 1.0, 1.0) * (1.0 - t) + Point3D::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: Point3D, radius: f32, r: &Ray) -> f32 {
    let oc = r.get_origin() - center;
    let dir = r.get_direction();
    let a = dir.length_squared();
    let c = oc.length_squared() - radius * radius;
    let half_b = dot(&oc, &dir);
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    (-half_b - discriminant.sqrt()) / a
}

fn write_ppm_image() {

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3D::new(0.0, 0.0, 0.0);
    let horizontal = Point3D::new(viewport_width, 0.0, 0.0);
    let vertical = Point3D::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Point3D::new(0.0, 0.0, focal_length);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let pixel_color = ray_color(r);
        }
    }
}

