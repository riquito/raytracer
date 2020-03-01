use crate::materials::Material;
use crate::ray::Ray;

use ndarray::Array1;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Array1<f64>,
    pub normal: Array1<f64>,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(material: &'a Material) -> Self {
        HitRecord {
            t: 0.,
            p: Array1::zeros(3),
            normal: Array1::zeros(3),
            material,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Array1<f64>) {
        let front_face = r.direction.dot(&outward_normal) < 0.;
        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Array1<f64>,
    pub radius: f64,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &r.origin - &self.center;
        let a = r.direction.dot(&r.direction);
        let half_b = oc.dot(&r.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0. {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let mut rec = HitRecord {
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: Array1::zeros(3),
                    material: &self.material,
                };

                let outward_normal = (&rec.p - &self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);

                return Some(rec);
            }

            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let mut rec = HitRecord {
                    t: temp,
                    p: r.point_at_parameter(temp),
                    normal: Array1::zeros(3),
                    material: &self.material,
                };

                let outward_normal = (&rec.p - &self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);

                return Some(rec);
            }
        }

        None
    }
}
