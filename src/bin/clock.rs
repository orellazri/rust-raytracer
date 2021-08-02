extern crate rust_raytracer as raytracer;

use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

use raytracer::{canvas::*, color::*, transformation::*, tuple::*};

fn point_to_canvas(canvas: &Canvas, point: &Tuple) -> (usize, usize) {
    let w = canvas.width as f64 / 2.0;
    let h = canvas.width as f64 / 2.0;

    let x = (w * point.x) + w;
    let y = (canvas.height as f64) - ((h * point.z) + h);
    (x as usize, y as usize)
}

fn main() {
    let mut canvas = Canvas::new(200, 200);
    let color = Color::white();

    let mut clock = Tuple::point(0.0, 0.0, 3.0 / 4.0);
    let (x, y) = point_to_canvas(&canvas, &clock);
    canvas.write_pixel(x, y, &color);

    for i in 1..12 {
        let transform = rotation_y(i as f64 * PI / 6.0);
        clock = transform * clock;
        let (x, y) = point_to_canvas(&canvas, &clock);
        canvas.write_pixel(x, y, &color);
    }

    println!("Starting to output ppm...");
    let mut file = File::create("output/clock.ppm").expect("Unable to create file");
    file.write_all(&canvas.to_ppm()[..]).expect("Unable to write data to file");
}
