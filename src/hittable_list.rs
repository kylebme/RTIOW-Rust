use std::vec::Vec;

use crate::hit::{Hit, HitRecord};
use crate::sphere::Sphere;
use crate::ray::Ray;

use enum_dispatch::enum_dispatch;

// This way of constructing a hittable list does not provide static dispatch of multiple types,
// because during construction, the explicit type must be stated.
// We need to be able to hold multiple types in one Vec.
// This means we need a constant sized container to put into Vec.
// Box would work but adds extra heap allocation.
// enum is used below. Don't know which is really better...
// pub struct MyHittableList<'a, T: Hit> {
//     list: &'a mut Vec<T>
// }


// Must explicitely list all hittable types.
// Implementing Hit is not sufficient.
// enum_dispatch will generate impl's for all members of enum
#[enum_dispatch]
pub enum Hittable {
    Sphere,
}


pub type HittableList = Vec<Hittable>;

// impl Hit for HittableList {
//     fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_record: &mut crate::hit::HitRecord) -> bool {
//         let mut hit_anything = false;
//         let mut closest_so_far = t_max;

//         for hittable in self {
//             let mut temp_rec = HitRecord::default();
//             if hittable.hit(r, t_min, closest_so_far, &mut temp_rec) {
//                 hit_anything = true;
//                 closest_so_far = temp_rec.t;
                
//                 hit_record = (*temp_rec).clone();

//                 // This seems dumb...
//                 // hit_record.t = temp_rec.t;
//                 // hit_record.p = temp_rec.p;
//                 // hit_record.front_face = temp_rec.front_face;
//                 // hit_record.normal = temp_rec.normal;

//             }
//         }
//         hit_anything
//     }
// }


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