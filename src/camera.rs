use crate::random_in_unit_disc;
use crate::ray::Ray;
use crate::MoreOps;
use ndarray::Array1;

pub struct Camera {
    lower_left_corner: Array1<f64>,
    horizontal: Array1<f64>,
    vertical: Array1<f64>,
    origin: Array1<f64>,
    u: Array1<f64>,
    v: Array1<f64>,
    w: Array1<f64>,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Array1<f64>,
        lookat: Array1<f64>,
        vup: Array1<f64>,
        vfov: f64, // field of view, top to bottom, in degrees
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.).tan();
        let half_width = aspect_ratio * half_height;
        let w = (&lookfrom - &lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);
        let lens_radius = aperture / 2.;

        let lower_left_corner = &lookfrom
            - &(half_width * focus_dist * &u)
            - (half_height * focus_dist * &v)
            - focus_dist * &w;
        let horizontal = 2. * half_width * focus_dist * &u;
        let vertical = 2. * half_height * focus_dist * &v;
        let origin = lookfrom;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Array1<f64> = self.lens_radius * random_in_unit_disc();
        let offset: Array1<f64> = &self.u * rd[0] + &self.v * rd[1];

        Ray {
            origin: &offset + &self.origin,
            direction: s * &self.horizontal + t * &self.vertical - &self.origin
                + &self.lower_left_corner
                - offset,
        }
    }
}
