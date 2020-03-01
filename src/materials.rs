use crate::hittable::HitRecord;
use crate::random_in_unit_sphere;
use crate::random_unit_vector;
use crate::ray::Ray;
use crate::MoreOps;
use ndarray::Array1;

pub trait MaterialCommon {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Scatter {
    pub attenuation: Array1<f64>,
    pub ray: Ray,
}

pub struct Lambertian {
    pub albedo: Array1<f64>,
}

impl MaterialCommon for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let scatter_direction = random_unit_vector() + &rec.normal;
        Some(Scatter {
            ray: Ray::new(rec.p.clone(), scatter_direction),
            attenuation: self.albedo.clone(),
        })
    }
}

pub struct Metal {
    pub albedo: Array1<f64>,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Array1<f64>, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

fn reflect(v: &Array1<f64>, n: &Array1<f64>) -> Array1<f64> {
    v - &(2. * v.dot(n) * n)
}

impl MaterialCommon for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(&r_in.direction.unit_vector(), &rec.normal);
        let scattered = Ray::new(
            rec.p.clone(),
            reflected + self.fuzz * random_in_unit_sphere(),
        );

        if scattered.direction.dot(&rec.normal) > 0. {
            Some(Scatter {
                ray: scattered,
                attenuation: self.albedo.clone(),
            })
        } else {
            None
        }
    }
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl MaterialCommon for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        match rec.material {
            Material::Lambertian(x) => x.scatter(r_in, rec),
            Material::Metal(x) => x.scatter(r_in, rec),
        }
    }
}
