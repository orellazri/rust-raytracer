extern crate rust_raytracer as raytracer;

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
    let mut proj = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0).normalized(),
    };
    let env = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    while proj.position.y > 0.0 {
        tick(&env, &mut proj);
        println!("{:?}", proj.position);
    }
}
