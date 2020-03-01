mod camera;
mod hittable;
mod hittable_list;
mod materials;
mod ray;

use crate::camera::Camera;
use crate::hittable::{Hittable, Sphere};
use crate::hittable_list::HittableList;
use crate::materials::{Lambertian, Material, MaterialCommon, Metal, Scatter};
use ray::Ray;

use minifb::{Key, Window, WindowOptions};
use ndarray::Array1;
use rand::Rng;
use rayon::prelude::*;
use std::error::Error;

const WIDTH: usize = 400;
const HEIGHT: usize = 200;

trait MoreOps {
    fn unit_vector(&self) -> Array1<f64>;
    fn squared_length(&self) -> f64;
    fn length(&self) -> f64;
}

impl MoreOps for Array1<f64> {
    fn unit_vector(&self) -> Array1<f64> {
        self / self.length()
    }
    fn squared_length(&self) -> f64 {
        (self * self).sum()
    }
    fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }
}

pub fn random_in_unit_sphere() -> Array1<f64> {
    let mut rng = rand::thread_rng();

    loop {
        let p: Array1<f64> = Array1::from(vec![
            rng.gen_range(-1., 1.),
            rng.gen_range(-1., 1.),
            rng.gen_range(-1., 1.),
        ]);

        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Array1<f64> {
    let mut rng = rand::thread_rng();

    let a: f64 = rng.gen_range(0., 2. * std::f64::consts::PI);
    let z: f64 = rng.gen_range(-1., 1.);
    let r = (1. - z * z).sqrt();

    Array1::from(vec![r * a.cos(), r * a.sin(), z])
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

fn color(r: &Ray, world: &dyn Hittable, depth: usize) -> Array1<f64> {
    if let Some(rec) = world.hit(r, 0.001, std::f64::MAX) {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth == 0 {
            return Array1::<f64>::zeros(3);
        }

        if let Some(Scatter {
            ray: scattered,
            attenuation,
        }) = rec.material.scatter(r, &rec)
        {
            attenuation * color(&scattered, world, depth - 1)
        } else {
            Array1::<f64>::zeros(3)
        }
    } else {
        let unit_direction = r.direction.unit_vector();
        let t: f64 = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - t) * Array1::from(vec![1., 1., 1.]) + t * Array1::from(vec![0.5, 0.7, 1.])
    }
}

pub fn draw_pixel(
    i: usize,
    width: usize,
    height: usize,
    cam: &Camera,
    world: &dyn Hittable,
) -> u32 {
    let mut rng = rand::thread_rng();
    let u = (i % width) as f64 / width as f64;
    let v = 1.0 - (i as f64 / width as f64 / height as f64);

    let mut col = Array1::from(vec![0., 0., 0.]);
    let samples_per_pixel = 100;
    let max_depth = 50;
    for _ in 0..samples_per_pixel {
        let tmp_u = u + rng.gen::<f64>() / width as f64;
        let tmp_v = v + rng.gen::<f64>() / height as f64;
        let r = cam.get_ray(tmp_u, tmp_v);

        col = col + color(&r, world, max_depth);
    }

    // Divide the color total by the number of samples and gamma-correct
    // for a gamma value of 2.0 (it's the following sqrt).
    col /= samples_per_pixel as f64;

    let ir = (256.0 * clamp(col[0].sqrt(), 0.0, 0.999)) as u32;
    let ig = (256.0 * clamp(col[1].sqrt(), 0.0, 0.999)) as u32;
    let ib = (256.0 * clamp(col[2].sqrt(), 0.0, 0.999)) as u32;
    (ir << 16) | (ig << 8) | ib
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut window = Window::new("Test", WIDTH, HEIGHT, WindowOptions::default())?;
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    window.limit_update_rate(Some(std::time::Duration::from_micros(1_000_000)));

    let lower_left_corner = Array1::from(vec![-2., -1., -1.]);
    let horizontal = Array1::from(vec![4., 0., 0.]);
    let vertical = Array1::from(vec![0., 2., 0.]);
    let origin = Array1::from(vec![0., 0., 0.]);
    let cam = Camera::new(lower_left_corner, horizontal, vertical, origin);

    let list = vec![
        Sphere {
            center: Array1::from(vec![0., 0., -1.]),
            radius: 0.5,
            material: Material::Lambertian(Lambertian {
                albedo: Array1::from(vec![0.7, 0.3, 0.3]),
            }),
        },
        Sphere {
            center: Array1::from(vec![0., -100.5, -1.]),
            radius: 100.,
            material: Material::Lambertian(Lambertian {
                albedo: Array1::from(vec![0.8, 0.8, 0.0]),
            }),
        },
        Sphere {
            center: Array1::from(vec![1., 0., -1.]),
            radius: 0.5,
            material: Material::Metal(Metal::new(Array1::from(vec![0.8, 0.6, 0.2]), 0.3)),
        },
        Sphere {
            center: Array1::from(vec![-1., 0., -1.]),
            radius: 0.5,
            material: Material::Metal(Metal::new(Array1::from(vec![0.8, 0.8, 0.8]), 1.0)),
        },
    ];

    let world = HittableList { list };
    let chunks = 1000;

    while window.is_open() && !window.is_key_down(Key::Enter) {
        let start = std::time::Instant::now();
        buffer
            .par_chunks_mut(chunks)
            .enumerate()
            .for_each(|(i_w, window)| {
                let mut i = i_w * chunks;
                for bi in window {
                    *bi = draw_pixel(i, WIDTH, HEIGHT, &cam, &world);
                    i += 1;
                }
            });

        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
        let duration = start.elapsed();

        println!("Time elapsed: {:?}", duration);
        println!("Sleep for a while");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    Ok(())
}
