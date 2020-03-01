use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList<T: Hittable> {
    pub list: Vec<Box<T>>,
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for elem in self.list.iter() {
            if elem.hit(r, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}
