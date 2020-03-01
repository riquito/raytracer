use crate::hittable::HitRecord;
use crate::random_in_unit_sphere;
use crate::random_unit_vector;
use crate::ray::Ray;
use crate::MoreOps;
use ndarray::Array1;
use rand::Rng;

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

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx }
    }
}

fn refract(uv: &Array1<f64>, n: &Array1<f64>, etai_over_etat: f64) -> Array1<f64> {
    let cos_theta = (-uv).dot(n);
    let r_out_parallel = etai_over_etat * (cos_theta * n + uv);
    let r_out_perp = -(1.0 - r_out_parallel.squared_length()).sqrt() * n;
    r_out_parallel + r_out_perp
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

fn ffmin(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

impl MaterialCommon for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let etai_over_etat: f64 = if rec.front_face {
            1. / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = ffmin((-&unit_direction).dot(&rec.normal), 1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let mut rng = rand::thread_rng();
        let reflect_prob = schlick(cos_theta, etai_over_etat);

        let scattered = if (etai_over_etat * sin_theta > 1.) || (rng.gen::<f64>() < reflect_prob) {
            let reflected = reflect(&unit_direction, &rec.normal);
            Ray::new(rec.p.clone(), reflected)
        } else {
            let refracted = refract(&unit_direction, &rec.normal, etai_over_etat);
            Ray::new(rec.p.clone(), refracted)
        };

        let attenuation = Array1::from(vec![1., 1., 1.]);

        Some(Scatter {
            ray: scattered,
            attenuation,
        })
    }
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl MaterialCommon for Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        match rec.material {
            Material::Lambertian(x) => x.scatter(r_in, rec),
            Material::Metal(x) => x.scatter(r_in, rec),
            Material::Dielectric(x) => x.scatter(r_in, rec),
        }
    }
}
