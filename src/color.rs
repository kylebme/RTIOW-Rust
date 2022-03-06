use crate::hit::Hit;
use crate::ray::*;
use crate::vec3::*;
use image::Rgb;
use rand::distributions::Uniform;
use rand::prelude::ThreadRng;
use crate::material::Material;

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
    if depth == 0 {
        return Vec3::zeros();
    }

    let option_rec = world.hit(r, 0.001, f64::INFINITY);
    if let Some(rec) = option_rec {
        let scatter_result_option = rec.mat_ref.scatter(r, &rec, rng, uniform);
        if let Some(scatter_result) = scatter_result_option {
            scatter_result.attenuation * ray_color_vec(&scatter_result.ray, world, rng, uniform, depth - 1)
        } else {
            Vec3::zeros()
        }
    } else {
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
