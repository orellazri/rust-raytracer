extern crate rust_raytracer as raytracer;

use std::fs::File;
use std::io::Write;

use raytracer::canvas::*;
use raytracer::color::*;
use raytracer::tuple::*;

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

#[derive(Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Environment, proj: &mut Projectile) {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    *proj = Projectile { position, velocity }
}

fn main() {
    let mut canvas = Canvas::new(400, 300);

    let mut proj = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalized() * 11.25,
    };
    let env = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    while proj.position.y > 0.0 {
        tick(&env, &mut proj);
        canvas.write_pixel(
            proj.position.x as usize,
            proj.position.y as usize,
            Color::red(),
        );
    }

    println!("Starting to output ppm...");
    let mut file = File::create("output/projectile.ppm").expect("Unable to create file");
    file.write_all(canvas.to_ppm().as_bytes())
        .expect("Unable to write data to file");
}
