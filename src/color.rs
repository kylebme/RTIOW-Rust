use image::Rgb;
use crate::vec3::*;
use crate::ray::*;
use crate::hit::Hit;

pub trait IntoColor {
    fn into_color(self) -> Rgb<u8>;
}

impl IntoColor for Vec3 {
    fn into_color(self) -> Rgb<u8> {
        Rgb([
            (255.0 * self.x) as u8, 
            (255.0 * self.y) as u8, 
            (255.0 * self.z) as u8
            ])
    }
}

impl IntoColor for [f64; 3] {
    fn into_color(self) -> Rgb<u8> {
        Rgb([
            (255.0 * self[0]) as u8, 
            (255.0 * self[1]) as u8, 
            (255.0 * self[2]) as u8
            ])
    }
}

pub fn ray_color_vec(r: &Ray, world: &impl Hit) -> Vec3 {
    let option_rec = world.hit(r, 0.0, f64::INFINITY);
    match option_rec {
        Some(rec) => {
            0.5 * (rec.normal + Vec3 {x:1.0, y:1.0, z:1.0})
        },
        None => {
            let unit_direction = r.direction.unit_vec();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t)*Vec3 {x:1.0, y:1.0, z:1.0} + t*Vec3 {x:0.5, y:0.7, z:1.0}
        }
    }
}