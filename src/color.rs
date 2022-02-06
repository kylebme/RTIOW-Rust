use crate::hit::Hit;
use crate::ray::*;
use crate::vec3::*;
use image::Rgb;
use rand::distributions::Uniform;
use rand::prelude::ThreadRng;

pub trait IntoColor {
    fn into_color(self) -> Rgb<u8>;
}

impl IntoColor for Vec3 {
    fn into_color(self) -> Rgb<u8> {
        Rgb([
            (255.0 * self.x) as u8,
            (255.0 * self.y) as u8,
            (255.0 * self.z) as u8,
        ])
    }
}

impl IntoColor for [f64; 3] {
    fn into_color(self) -> Rgb<u8> {
        Rgb([
            (255.0 * self[0]) as u8,
            (255.0 * self[1]) as u8,
            (255.0 * self[2]) as u8,
        ])
    }
}

pub fn ray_color_vec(
    r: &Ray,
    world: &impl Hit,
    rng: &mut ThreadRng,
    uniform: Uniform<f64>,
    depth: u32,
) -> Vec3 {
    if depth <= 0 {
        return Vec3::zeros();
    }

    let option_rec = world.hit(r, 0.001, f64::INFINITY);
    match option_rec {
        Some(rec) => {
            // let target = rec.p + rec.normal + Vec3::random_unit_vector(rng, uniform);
            let target = rec.p + Vec3::random_in_hemisphere(rng, uniform, rec.normal);
            0.5 * ray_color_vec(
                &Ray {
                    origin: rec.p,
                    direction: target - rec.p,
                },
                world,
                rng,
                uniform,
                depth - 1,
            )
        }
        None => {
            let unit_direction = r.direction.unit_vec();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vec3::ones()
                + t * Vec3 {
                    x: 0.5,
                    y: 0.7,
                    z: 1.0,
                }
        }
    }
}
