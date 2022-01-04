// use std::fs::File;
// use std::io::prelude::*;
use std::time::{Instant};
use image::{RgbImage};

mod vec3;
use vec3::{Vec3};

mod ray;
use ray::*;

mod hit;

mod hittable_list;
use hittable_list::HittableList;

mod sphere;
use sphere::Sphere;

mod color;
use color::*;

fn main() -> std::io::Result<()> {
    println!("Start");
    let start = Instant::now();
    
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio).ceil() as usize;
    
    // World
    let mut world = HittableList::new();

    world.push(Sphere {
        center: Vec3 { x: 0.0, y: 0.0, z: -1.0 },
        radius: 0.5
    }.into());

    world.push(Sphere {
        center: Vec3 { x: 0.0, y: -100.5, z: -1.0 },
        radius: 100.0
    }.into());


    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    
    let origin = Vec3 {x:0.0, y:0.0, z:0.0};
    let horizontal = Vec3 {x:viewport_width, y:0.0, z:0.0};
    let vertical = Vec3 {x:0.0, y:viewport_height, z:0.0};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3 {x:0.0, y: 0.0, z: focal_length};
    
    let mut img = RgbImage::new(
        image_width as u32,
        image_height as u32);

    for j in 0..image_height {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = 1.0 - (j as f64 / (image_height - 1) as f64);

            let r = Ray {
                orig: origin,
                dir: lower_left_corner + u*horizontal + v*vertical - origin
            };

            let pixel_color = ray_color(&r, &world);
            img.put_pixel(i as u32, j as u32, pixel_color);
        }
    }

    img.save("./image.tiff").unwrap();

    println!("Time elapsed: {:?}", start.elapsed());
    
    Ok(())
}

