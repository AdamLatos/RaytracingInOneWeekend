use super::hittable::*;
use super::ray::*;
use super::vec3::*;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.orig - self.center;
        let a = ray.dir.len_squared();
        let half_b = dot(oc, ray.dir);
        let c = oc.len_squared() - (self.radius * self.radius);
        let delta = half_b * half_b - a * c;

        if delta > 0.0 {
            let root = delta.sqrt();
            let t1 = (-half_b - root) / a;
            if t1 < t_max && t1 > t_min {
                record.t = t1;
                record.p = ray.at(t1);
                record.normal = (record.p - self.center) / self.radius;
                return true
            }  
            let t2 = (-half_b + root) / a;
            if t2 < t_max && t2 > t_min {
                record.t = t2;
                record.p = ray.at(t2);
                record.normal = (record.p - self.center) / self.radius;
                return true
            } 
        }
        false
    }
}
