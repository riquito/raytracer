use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList<T: Hittable> {
    pub list: Vec<T>,
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec = None;
        for elem in self.list.iter() {
            if let Some(new_rec) = elem.hit(r, t_min, closest_so_far) {
                closest_so_far = new_rec.t;
                rec = Some(new_rec);
            }
        }

        rec
    }
}
