// use std::fs::File;
// use std::io::prelude::*;
use image::{ImageBuffer, Rgb, RgbImage};
use rand::distributions::{Distribution, Uniform};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use indicatif::ParallelProgressIterator;

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

mod uniform_wrapper;
use uniform_wrapper::*;

fn main() -> std::io::Result<()> {
    println!("Start");
    let start = Instant::now();

    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 500;
    let image_height: u32 = (image_width as f64 / aspect_ratio).ceil() as u32;
    const SAMPLES_PER_PIXEL: u32 = 50;
    const MAX_DEPTH: u32 = 50;

    // World
    let world = random_scene();

    // Camera
    let cam = Camera::new(
        Vec3 {
            x: 13.0,
            y: 2.0,
            z: 3.0,
        },
        Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        Vec3 {
            x: 0.,
            y: 1.,
            z: 0.,
        },
        20.0,
        aspect_ratio,
        0.10,
        10.0,
    );

    let img = render(
        world,
        cam,
        image_width as u32,
        image_height as u32,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
    );
    img.save("./image.bmp").unwrap();

    println!("Time elapsed: {:?}", start.elapsed());

    Ok(())
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let mut unigen0_1 = UniGen0_1::new();
    let mut rng = thread_rng();

    world.push(
        Sphere {
            center: Vec3 {
                x: 0.0,
                y: -1000.0,
                z: 0.0,
            },
            radius: 1000.0,
            material: Lambertian {
                albedo: Vec3 {
                    x: 0.5,
                    y: 0.5,
                    z: 0.5,
                },
            }
            .into(),
        }
        .into(),
    );

    let scene_center = Vec3 {
        x: 4.0,
        y: 0.2,
        z: 0.0,
    };

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen_range(0.0..1.0);
            let center = Vec3 {
                x: a as f64 + rng.gen_range(0.0..0.9),
                y: 0.2,
                z: b as f64 + rng.gen_range(0.0..0.9),
            };

            if (center - scene_center).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_unit_vector(&mut unigen0_1)
                        * Vec3::random_unit_vector(&mut unigen0_1);
                    let sphere_material = Lambertian { albedo }.into();
                    world.push(
                        Sphere {
                            center,
                            radius: 0.2,
                            material: sphere_material,
                        }
                        .into(),
                    );
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_unit_vector(&mut UniGenUntyped::new(0.5, 1.0));
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Metal { albedo, fuzz }.into();
                    world.push(
                        Sphere {
                            center,
                            radius: 0.2,
                            material: sphere_material,
                        }
                        .into(),
                    );
                } else {
                    let sphere_material = Dielectric { ir: 1.5 }.into();
                    world.push(
                        Sphere {
                            center,
                            radius: 0.2,
                            material: sphere_material,
                        }
                        .into(),
                    );
                }
            }
        }
    }

    world.push(
        Sphere {
            center: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            radius: 1.0,
            material: Dielectric { ir: 1.5 }.into(),
        }
        .into(),
    );

    world.push(
        Sphere {
            center: Vec3 {
                x: -4.0,
                y: 1.0,
                z: 0.0,
            },
            radius: 1.0,
            material: Lambertian {
                albedo: Vec3 {
                    x: 0.4,
                    y: 0.2,
                    z: 0.1,
                },
            }
            .into(),
        }
        .into(),
    );

    world.push(
        Sphere {
            center: Vec3 {
                x: 4.0,
                y: 1.0,
                z: 0.0,
            },
            radius: 1.0,
            material: Metal {
                albedo: Vec3 {
                    x: 0.7,
                    y: 0.6,
                    z: 0.5,
                },
                fuzz: 0.0,
            }
            .into(),
        }
        .into(),
    );

    world
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

    (0..image_height as u64).into_par_iter().for_each(|j| {
        let mut unigen0_1 = UniGen0_1::new();
        let mut unigen_neg1_1 = UniGenNeg1_1::new();

        for i in 0..image_width {
            let mut pixel_color_vec = Vec3::zeros();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + unigen0_1.sample()) / (image_width - 1) as f64;
                let v =
                    1.0 - ((j as f64 + unigen0_1.sample()) / (image_height - 1) as f64);

                let r = cam.get_ray(u, v, &mut unigen_neg1_1);

                pixel_color_vec += ray_color_vec(&r, &world, &mut unigen0_1, &mut unigen_neg1_1, max_depth);
            }
            let scaled_pixel_color_vec = (pixel_color_vec / samples_per_pixel as f64).sqrt();

            let mut img = img.lock().unwrap();
            img.put_pixel(i as u32, j as u32, scaled_pixel_color_vec.into_color());
        }
    });

    let lock = Arc::try_unwrap(img).expect("img has multiple owners");
    lock.into_inner().expect("Mutex cannot be locked")
}
