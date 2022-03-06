use crate::material;
use crate::vec3::*;
use crate::ray::*;
use crate::material::MaterialEnum;

use crate::sphere::Sphere;

use enum_dispatch::enum_dispatch;

// #[derive(Debug, Default, Clone)]
pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat_ref: &'a MaterialEnum,
    pub t: f64,
    pub front_face: bool
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Vec3, t: f64, mat_ref: &'a MaterialEnum, outward_normal: Vec3, r: &Ray) -> HitRecord<'a> {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {outward_normal} else {-outward_normal};
        HitRecord { p, normal, mat_ref: &mat_ref, t, front_face }
    }
}

#[enum_dispatch(Hittable)]
pub trait Hit {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}


// Must explicitly list all hittable types.
// Implementing Hit is not sufficient.
// enum_dispatch will generate impl's for all members of enum
#[enum_dispatch]
pub enum Hittable {
    Sphere,
}

pub type HittableList = Vec<Hittable>;

impl Hit for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_t_so_far = t_max;
        let mut closest_hit: Option<HitRecord> = None;

        for hittable in self {
            let option_rec = hittable.hit(r, t_min, closest_t_so_far);
            match option_rec {
                Some(rec) => {
                    closest_t_so_far = rec.t;
                    closest_hit = Some(rec);
                },
                None => ()
            }
        }
        closest_hit
    }
}