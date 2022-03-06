use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Reflect, Vec3, VecLength, VecProducts};

use enum_dispatch::enum_dispatch;
use rand::distributions::Uniform;
use rand::prelude::ThreadRng;

pub struct ScatterResult {
    pub attenuation: Vec3,
    pub ray: Ray,
}

#[enum_dispatch(MaterialEnum)]
pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        rng: &mut ThreadRng,
        uniform: Uniform<f64>,
    ) -> Option<ScatterResult>;
}

#[enum_dispatch]
pub enum MaterialEnum {
    Lambertian,
    Metal
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_rec: &HitRecord,
        rng: &mut ThreadRng,
        uniform: Uniform<f64>,
    ) -> Option<ScatterResult> {
        let scatter_direction_maybe =
            hit_rec.normal + Vec3::random_in_unit_sphere(rng, uniform);

        let scatter_direction = if scatter_direction_maybe.near_zero() {
            hit_rec.normal
        } else {
            scatter_direction_maybe
        };

        let scattered_ray = Ray {
            origin: hit_rec.p,
            direction: scatter_direction,
        };

        Some(ScatterResult {
            attenuation: self.albedo,
            ray: scattered_ray,
        })
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64 // must be between 0-1
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        rng: &mut ThreadRng,
        uniform: Uniform<f64>,
    ) -> Option<ScatterResult> {
        let reflected = ray_in.direction.unit_vec().reflect(hit_rec.normal);
        let scattered = Ray {
            origin: hit_rec.p,
            direction: reflected + self.fuzz * Vec3::random_in_unit_sphere(rng, uniform),
        };
        if scattered.direction.dot(hit_rec.normal) > 0.0 {
            Some(ScatterResult {
                attenuation: self.albedo,
                ray: scattered,
            })
        } else {
            None
        }
    }
}
