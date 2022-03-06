use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Reflect, Refract, Vec3, VecLength, VecProducts};

use enum_dispatch::enum_dispatch;
use rand::{
    distributions::Uniform,
    prelude::{Distribution, ThreadRng},
};

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
    Metal,
    Dielectric,
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
        let scatter_direction_maybe = hit_rec.normal + Vec3::random_in_unit_sphere(rng, uniform);

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
    pub fuzz: f64, // must be between 0-1
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

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        let r0_squared = r0 * r0;

        r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        rng: &mut ThreadRng,
        uniform: Uniform<f64>,
    ) -> Option<ScatterResult> {
        let refraction_ratio = if hit_rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = ray_in.direction.unit_vec();
        let cos_theta = (-unit_direction).dot(hit_rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let reflectance = Dielectric::reflectance(cos_theta, refraction_ratio);

        let cannot_refract = refraction_ratio * sin_theta > 1.0
            || reflectance > uniform.sample(rng);

        let direction = if cannot_refract {
            unit_direction.reflect(hit_rec.normal)
        } else {
            unit_direction.refract(hit_rec.normal, refraction_ratio)
        };

        Some(ScatterResult {
            attenuation: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            ray: Ray {
                origin: hit_rec.p,
                direction,
            },
        })
    }
}
