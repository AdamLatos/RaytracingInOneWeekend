use super::ray::*;
use super::vec3::*;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(1.0, 0.0, 0.0),
            t: 0.0
        }
    }
}

pub trait Hittable {
    fn hit(self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}