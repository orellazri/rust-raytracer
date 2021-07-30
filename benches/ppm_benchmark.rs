extern crate rust_raytracer as raytracer;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::Rng;

use raytracer::{canvas::*, color::*};

fn criterion_benchmark(c: &mut Criterion) {
    let mut canvas = Canvas::new(900, 500);
    let mut rng = rand::thread_rng();
    let r: f64 = rng.gen_range(0..101) as f64 / 100.0;
    let g: f64 = rng.gen_range(0..101) as f64 / 100.0;
    let b: f64 = rng.gen_range(0..101) as f64 / 100.0;

    for y in 0..canvas.height {
        for x in 0..canvas.width {
            canvas.write_pixel(x, y, Color::new(r, g, b));
        }
    }

    c.bench_function("ppm", |b| b.iter(|| canvas.to_ppm()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
