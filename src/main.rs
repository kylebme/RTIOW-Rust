// use std::fs::File;
// use std::io::prelude::*;
use image::RgbImage;
use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Uniform};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

mod vec3;
use vec3::*;

mod ray;
use ray::*;

mod hit;

mod hittable_list;
use hittable_list::HittableList;

mod sphere;
use sphere::Sphere;

mod color;
use color::*;

mod camera;
use camera::*;

fn main() -> std::io::Result<()> {
    println!("Start");
    let start = Instant::now();

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 2000;
    let image_height: usize = (image_width as f64 / aspect_ratio).ceil() as usize;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new();

    world.push(
        Sphere {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            radius: 0.5,
        }
        .into(),
    );

    world.push(
        Sphere {
            center: Vec3 {
                x: 0.0,
                y: -100.5,
                z: -1.0,
            },
            radius: 100.0,
        }
        .into(),
    );

    // Camera
    let cam = Camera::new(Vec3::zeros(), aspect_ratio, 2.0, 1.0);

    // Render
    let img = Arc::new(Mutex::new(
        RgbImage::new(
        image_width as u32,
        image_height as u32,
    )));

    const RAND_RANGE: std::ops::Range<f64> = 0.0..1.0;

    (0..image_height).into_par_iter().for_each(|j| {
        let mut rng = thread_rng();
        let uniform = Uniform::from(RAND_RANGE);
        for i in 0..image_width {
            let mut pixel_color_vec = Vec3::zeros();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + uniform.sample(&mut rng)) / (image_width - 1) as f64;
                let v = 1.0 - ((j as f64 + uniform.sample(&mut rng)) / (image_height - 1) as f64);

                let r = cam.get_ray(u, v);

                pixel_color_vec += ray_color_vec(&r, &world, &mut rng, uniform, MAX_DEPTH);
            }
            let scaled_pixel_color_vec = (pixel_color_vec / SAMPLES_PER_PIXEL as f64).sqrt();

            let mut img = img.lock().unwrap();
            img.put_pixel(i as u32, j as u32, scaled_pixel_color_vec.into_color());
        }
    });

    let img = img.lock().unwrap();
    img.save("./image.tiff").unwrap();

    println!("Time elapsed: {:?}", start.elapsed());

    Ok(())
}
