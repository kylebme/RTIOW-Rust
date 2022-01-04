use std::vec::Vec;

use crate::hit::{Hit, HitRecord};
use crate::sphere::Sphere;
use crate::ray::Ray;

use enum_dispatch::enum_dispatch;

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