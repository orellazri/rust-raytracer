extern crate rust_raytracer as raytracer;

use criterion::{criterion_group, criterion_main, Criterion};

use raytracer::{canvas::*, color::*, intersection, ray::Ray, sphere::Sphere, tuple::Tuple};

fn criterion_benchmark(c: &mut Criterion) {
    let red = Color::red();
    let sphere = Sphere::new();

    let canvas_pixels = 300;
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_size = 7.0;
    let wall_z = 10.0;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

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
        }
    }

    c.bench_function("sphere", |b| b.iter(|| canvas.to_ppm()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
