use crate::ray::Ray;

use ndarray::Array1;

pub struct HitRecord {
    pub t: f64,
    pub p: Array1<f64>,
    pub normal: Array1<f64>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.,
            p: Array1::zeros(3),
            normal: Array1::zeros(3),
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Array1<f64>,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = &r.origin - &self.center;
        let a = r.direction.dot(&r.direction);
        let b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0. {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (&rec.p - &self.center) / self.radius;
                return true;
            }

            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (&rec.p - &self.center) / self.radius;
                return true;
            }
        }
        false
    }
}
