use crate::matrix::Matrix;

#[derive(Debug)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub transform: Matrix,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::identity(),
        }
    }

    pub fn half_width(&self) -> f64 {
        let aspect = self.hsize / self.vsize;
        let half_view = f64::tan(self.field_of_view / 2.0);
        if aspect >= 1 {
            half_view
        } else {
            half_view * (aspect as f64)
        }
    }

    pub fn half_height(&self) -> f64 {
        let aspect = self.hsize / self.vsize;
        let half_view = f64::tan(self.field_of_view / 2.0);
        if aspect >= 1 {
            half_view / (aspect as f64)
        } else {
            half_view
        }
    }

    pub fn pixel_size(&self) -> f64 {
        (self.half_width() * 2.0) / (self.hsize as f64)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::utils::floats_equal;

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

        assert!(floats_equal(c.pixel_size(), 0.01));
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert!(floats_equal(c.pixel_size(), 0.01));
    }
}
