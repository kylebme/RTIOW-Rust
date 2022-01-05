use crate::vec3::*;
use crate::ray::*;

use enum_dispatch::enum_dispatch;

#[derive(Debug, Default, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new(p: Vec3, t: f64, outward_normal: Vec3, r: &Ray) -> HitRecord {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {outward_normal} else {-outward_normal};
        HitRecord { p, normal, t, front_face }
    }
}

// somehow this knows about Hittable, even though it's not included in this file
#[enum_dispatch(Hittable)]
pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

