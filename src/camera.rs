use std::sync::Mutex;

use indicatif::ProgressBar;
use itertools::Itertools;
use rayon::prelude::*;

use crate::{canvas::Canvas, matrix::Matrix, ray::Ray, tuple::Tuple, world::World};

#[derive(Debug)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transform: Matrix,
    pub half_width: f64,
    pub half_height: f64,
    pub pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = (hsize as f64 / vsize as f64) as f64;
        let half_width: f64;
        let half_height: f64;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / (hsize as f64);

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity(),
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.transform.inverse() * Tuple::point(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * Tuple::point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalized();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Mutex<Canvas> {
        println!("Raytracing {} pixels...", self.vsize * self.hsize);
        let progress = ProgressBar::new((self.vsize * self.hsize) as u64);
        progress.set_draw_rate(5);

        let canvas_mutex = Mutex::new(Canvas::new(self.hsize, self.vsize));

        (0..self.hsize) // x
            .cartesian_product(0..self.vsize) // y
            .par_bridge()
            .for_each(|(x, y)| {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);

                let mut canvas = canvas_mutex.lock().unwrap();
                canvas.write_pixel(x, y, &color);

                progress.inc(1);
            });

        progress.finish();

        canvas_mutex
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};

    use crate::{
        color::Color,
        transformation::{self, view_transform},
        utils::floats_equal,
    };

    use super::*;

    #[test]
    fn construct_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;
        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert!(floats_equal(c.field_of_view, PI / 2.0));
        assert_eq!(c.transform, Matrix::identity());
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);

        assert!(floats_equal(c.pixel_size, 0.01));
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert!(floats_equal(c.pixel_size, 0.01));
    }

    #[test]
    fn consturct_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn consturct_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn consturct_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = transformation::rotation_y(PI / 4.0) * transformation::translation(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0.0, 2.0, -5.0));
        assert_eq!(r.direction, Tuple::vector(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0));
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Tuple::point(0.0, 0.0, -5.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        c.transform = view_transform(from, to, up);
        let canvas_mutex = c.render(&w);
        let canvas = canvas_mutex.lock().unwrap();

        assert_eq!(canvas.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
