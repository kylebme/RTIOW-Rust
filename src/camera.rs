use crate::ray::Ray;
use crate::vec3::{Vec3, VecLength, VecProducts};
use crate::uniform_wrapper::*;
use rand::{prelude::ThreadRng, distributions::Uniform};

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        v_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = v_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vec();
        let u = v_up.cross(w).unit_vec();
        let v = w.cross(u);

        let origin = look_from;

        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, unigen: &mut UniGenNeg1_1) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(unigen);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin - offset,
        }
    }
}
