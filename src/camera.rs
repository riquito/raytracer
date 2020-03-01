use crate::ray::Ray;
use crate::MoreOps;
use ndarray::Array1;

pub struct Camera {
    lower_left_corner: Array1<f64>,
    horizontal: Array1<f64>,
    vertical: Array1<f64>,
    origin: Array1<f64>,
}

impl Camera {
    pub fn new(
        lookfrom: Array1<f64>,
        lookat: Array1<f64>,
        vup: Array1<f64>,
        vfov: f64, // field of view, top to bottom, in degrees
        aspect_ratio: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.).tan();
        let half_width = aspect_ratio * half_height;
        let w = (&lookfrom - &lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let lower_left_corner = &lookfrom - &(half_width * &u) - (half_height * &v) - &w;
        let horizontal = 2. * half_width * u;
        let vertical = 2. * half_height * v;
        let origin = lookfrom;

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
