extern crate rust_raytracer as raytracer;

use std::fs::File;
use std::io::Write;

use indicatif::ProgressBar;
use raytracer::{canvas::*, color::*, intersection, ray::Ray, sphere::Sphere, tuple::*};

fn main() {
    let red = Color::red();
    let sphere = Sphere::new();

    let canvas_pixels = 512;
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_size = 7.0;
    let wall_z = 10.0;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    println!("Raytracing {} pixels...", canvas_pixels.pow(2));
    let progress = ProgressBar::new(canvas_pixels.pow(2) as u64);
    progress.set_draw_rate(5);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * (x as f64);
            let position = Tuple::point(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin, (position - ray_origin).normalized());
            let xs = sphere.intersect(&r);

            if let Some(_hit) = intersection::hit(&xs) {
                canvas.write_pixel(x, y, red);
            }

            progress.inc(1);
        }
    }

    progress.finish();

    println!("Starting to output ppm...");
    let mut file = File::create("output/sphere.ppm").expect("Unable to create file");
    file.write_all(&canvas.to_ppm()[..]).expect("Unable to write data to file");
}
