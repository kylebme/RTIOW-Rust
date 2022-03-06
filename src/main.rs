// use std::fs::File;
// use std::io::prelude::*;
use image::{ImageBuffer, Rgb, RgbImage};
use rand::distributions::{Distribution, Uniform};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;

mod vec3;
use vec3::*;

mod ray;
use ray::*;

mod hit;
use hit::HittableList;

mod sphere;
use sphere::Sphere;

mod color;
use color::*;

mod camera;
use camera::*;

mod material;
use material::*;

fn main() -> std::io::Result<()> {
    println!("Start");
    let start = Instant::now();

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 1000;
    let image_height: u32 = (image_width as f64 / aspect_ratio).ceil() as u32;
    const SAMPLES_PER_PIXEL: u32 = 200;
    const MAX_DEPTH: u32 = 50;

    // World
    let mut world = HittableList::new();

    world.push(
        Sphere {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            radius: -0.5,
            material: Dielectric {
                ir: 1.5
            }.into()
        }
        .into(),
    );
    world.push(
        Sphere {
            center: Vec3 {
                x: -1.0,
                y: 0.0,
                z: -2.0,
            },
            radius: 0.5,
            material: Metal {
                albedo: Vec3 { x: 0.1, y: 0.1, z: 0.5 },
                fuzz: 0.1
            }.into()
        }
        .into(),
    );
    world.push(
        Sphere {
            center: Vec3 {
                x: 1.0,
                y: 0.0,
                z: -2.0,
            },
            radius: 0.5,
            material: Metal {
                albedo: Vec3 { x: 0.5, y: 0.0, z: 0.5 },
                fuzz: 0.5
            }.into()
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
            material: Lambertian {
                albedo: Vec3 { x: 0.9, y: 0.9, z: 0.9 }
            }.into()
        }
        .into(),
    );

    for i in -5..5+1 {
        world.push(Sphere {
            center: Vec3 {
                x: i as f64 * 0.50,
                y: 0.10,
                z: -0.80
            },
            radius: 0.1,
            material: Lambertian {
                albedo: Vec3 { x: 0.5, y: 0.5, z: 0.5 }
            }.into()
        }.into()
        );
    }

    // Camera
    let cam = Camera::new(Vec3::zeros(), aspect_ratio, 2.0, 1.0);

    let img = render(
        world,
        cam,
        image_width as u32,
        image_height as u32,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH
    );
    img.save("./image.bmp").unwrap();

    println!("Time elapsed: {:?}", start.elapsed());

    Ok(())
}

fn render(
    world: HittableList,
    cam: Camera,
    image_width: u32,
    image_height: u32,
    samples_per_pixel: u32,
    max_depth: u32,
) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    
    // Render
    let img = Arc::new(Mutex::new(RgbImage::new(
        image_width as u32,
        image_height as u32,
    )));

    const RAND_RANGE: std::ops::Range<f64> = 0.0..1.0;

    (0..image_height).into_par_iter().for_each(|j| {
        let mut rng = thread_rng();
        let uniform = Uniform::from(RAND_RANGE);
        for i in 0..image_width {
            let mut pixel_color_vec = Vec3::zeros();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + uniform.sample(&mut rng)) / (image_width - 1) as f64;
                let v = 1.0 - ((j as f64 + uniform.sample(&mut rng)) / (image_height - 1) as f64);

                let r = cam.get_ray(u, v);

                pixel_color_vec += ray_color_vec(&r, &world, &mut rng, uniform, max_depth);
            }
            let scaled_pixel_color_vec = (pixel_color_vec / samples_per_pixel as f64).sqrt();

            let mut img = img.lock().unwrap();
            img.put_pixel(i as u32, j as u32, scaled_pixel_color_vec.into_color());
        }
    });

    let lock = Arc::try_unwrap(img).expect("img has multiple owners"); 
    lock.into_inner().expect("Mutex cannot be locked")
}
