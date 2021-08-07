extern crate rust_raytracer as raytracer;

use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

use raytracer::camera::Camera;
use raytracer::transformation::{self, view_transform};
use raytracer::world::World;
use raytracer::{color::*, light::*, sphere::Sphere, tuple::*};

fn main() {
    let mut floor = Sphere::new();
    floor.transform = transformation::scaling(10.0, 0.01, 10.0);
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = Sphere::new();
    left_wall.transform = transformation::translation(0.0, 0.0, 5.0)
        * transformation::rotation_y(-PI / 4.0)
        * transformation::rotation_x(PI / 2.0)
        * transformation::scaling(10.0, 0.01, 10.0);
    left_wall.material = floor.material;

    let mut right_wall = Sphere::new();
    right_wall.transform = transformation::translation(0.0, 0.0, 5.0)
        * transformation::rotation_y(PI / 4.0)
        * transformation::rotation_x(PI / 2.0)
        * transformation::scaling(10.0, 0.01, 10.0);
    right_wall.material = floor.material;

    let mut middle = Sphere::new();
    middle.transform = transformation::translation(-0.5, 1.0, 0.5);
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::new();
    right.transform = transformation::translation(1.5, 0.5, -0.5) * transformation::scaling(0.5, 0.5, 0.5);
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::new();
    left.transform = transformation::translation(-1.5, 0.33, -0.75) * transformation::scaling(0.33, 0.33, 0.33);
    left.material.color = Color::new(0.317, 0.623, 0.929);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut world = World::new();
    world.light = Some(PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::white()));
    world.objects.push(floor);
    world.objects.push(left_wall);
    world.objects.push(right_wall);
    world.objects.push(middle);
    world.objects.push(left);
    world.objects.push(right);

    let mut camera = Camera::new(900, 400, PI / 3.0);
    camera.transform = view_transform(Tuple::point(0.0, 1.5, -5.0), Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));

    let canvas_mutex = camera.render(&world);
    let canvas = canvas_mutex.lock().unwrap();

    println!("Starting to output ppm...");
    let mut file = File::create("output/world.ppm").expect("Unable to create file");
    file.write_all(&canvas.to_ppm()[..]).expect("Unable to write data to file");
}
