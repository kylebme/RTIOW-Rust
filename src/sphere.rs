use crate::vec3::*;
use crate::ray::*;
use crate::hit::*;
use crate::material::MaterialEnum;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: MaterialEnum
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let origin_to_center = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = origin_to_center.dot(r.direction);
        let c = origin_to_center.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let record_p = r.at(root);
        let outward_normal = (record_p - self.center) / self.radius;

        let hit_record = HitRecord::new(record_p, root, &self.material, outward_normal, r);

        Some(hit_record)
    }
}