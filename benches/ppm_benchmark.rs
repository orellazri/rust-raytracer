extern crate rust_raytracer as raytracer;

use criterion::{criterion_group, criterion_main, Criterion};

use raytracer::{canvas::*, color::*};

fn criterion_benchmark(c: &mut Criterion) {
    let mut canvas = Canvas::new(900, 500);
    let red = Color::red();
    for y in 0..canvas.height {
        for x in 0..canvas.width {
            canvas.write_pixel(x, y, red);
        }
    }

    c.bench_function("ppm", |b| b.iter(|| canvas.to_ppm()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
