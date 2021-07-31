extern crate rust_raytracer as raytracer;

use std::io::Write;
use std::{fs::File, sync::Mutex};

use indicatif::ProgressBar;
use itertools::Itertools;
use rayon::prelude::*;
use raytracer::{canvas::*, color::*, intersection, ray::Ray, sphere::Sphere, tuple::*};

fn main() {
    let red = Color::red();
    let sphere = Sphere::new();

    let canvas_pixels = 2000;
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_size = 10.0;
    let wall_z = 5.0;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let canvas_mutex = Mutex::new(Canvas::new(canvas_pixels, canvas_pixels));

    println!("Raytracing {} pixels...", canvas_pixels.pow(2));
    let progress = ProgressBar::new(canvas_pixels.pow(2) as u64);
    progress.set_draw_rate(5);

    (0..canvas_pixels) // x
        .cartesian_product(0..canvas_pixels) // y
        .par_bridge()
        .for_each(|(x, y)| {
            let world_y = half - pixel_size * (y as f64);
            let world_x = -half + pixel_size * (x as f64);
            let position = Tuple::point(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalized());
            let xs = sphere.intersect(&r);

            if intersection::hit(&xs) != None {
                let mut canvas = canvas_mutex.lock().unwrap();
                canvas.write_pixel(x, y, red);
            }

            progress.inc(1);
        });

    progress.finish();

    let canvas = canvas_mutex.lock().unwrap();
    println!("Starting to output ppm...");
    let mut file = File::create("output/sphere.ppm").expect("Unable to create file");
    file.write_all(&canvas.to_ppm()[..]).expect("Unable to write data to file");

    drop(canvas);
}
