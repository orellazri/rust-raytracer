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

enum Pixel {
    Coordinate { x: usize, y: usize },
    OutOfBounds,
}

impl Pixel {
    pub fn from_point_for_canvas(point: Tuple, canvas: &Canvas) -> Pixel {
        if !point.is_point() {
            panic!("Given tuple is not a point. Point needed for conversion to screen space.");
        }

        let rx = point.x.round();
        let ry = point.y.round();

        let ux = rx as usize;
        let uy = ry as usize;

        if rx.is_sign_negative() || ry.is_sign_negative() || ux > canvas.width || uy > canvas.height
        {
            return Pixel::OutOfBounds;
        }

        let screen_x = ux;
        let screen_y = canvas.height - uy;

        Pixel::Coordinate {
            x: screen_x,
            y: screen_y,
        }
    }
}

fn tick(env: &Environment, proj: &mut Projectile) {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    *proj = Projectile { position, velocity }
}

fn main() {
    let mut canvas = Canvas::new(900, 500);

    let mut proj = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalized() * 11.25,
    };
    let env = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let red = Color::red();

    while proj.position.y > 0.0 {
        tick(&env, &mut proj);

        match Pixel::from_point_for_canvas(proj.position, &canvas) {
            Pixel::Coordinate { x, y } => {
                canvas.write_pixel(x, y, red);
            }
            Pixel::OutOfBounds => {}
        }
    }

    println!("Starting to output ppm...");
    let mut file = File::create("output/projectile.ppm").expect("Unable to create file");
    file.write_all(&canvas.to_ppm()[..])
        .expect("Unable to write data to file");
}
