use crate::ray::Ray;
use ndarray::Array1;

pub struct Camera {
    lower_left_corner: Array1<f64>,
    horizontal: Array1<f64>,
    vertical: Array1<f64>,
    origin: Array1<f64>,
}

impl Camera {
    pub fn new(
        lower_left_corner: Array1<f64>,
        horizontal: Array1<f64>,
        vertical: Array1<f64>,
        origin: Array1<f64>,
    ) -> Camera {
        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let ray_h: Array1<f64> = u * &self.horizontal;
        let ray_v: Array1<f64> = v * &self.vertical;

        Ray {
            origin: self.origin.clone(),
            direction: ray_h + ray_v + &self.lower_left_corner - &self.origin,
        }
    }
}
