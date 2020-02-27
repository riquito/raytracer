use minifb::{Key, Window, WindowOptions};
use ndarray::Array1;
use std::error::Error;

const WIDTH: usize = 640;
const HEIGHT: usize = 400;

struct Ray<'a> {
    origin: &'a Array1<f64>,
    direction: &'a Array1<f64>,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Array1<f64>, direction: &'a Array1<f64>) -> Ray<'a> {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f64) -> Array1<f64> {
        (t * self.direction) + self.origin
    }
}

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

fn color(r: &Ray) -> Array1<f64> {
    let unit_direction = r.direction.unit_vector();
    let t: f64 = 0.5 * (unit_direction[1] + 1.0);
    (1.0 - t) * Array1::from(vec![1., 1., 1.]) + t * Array1::from(vec![0.5, 0.7, 1.])
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut window = Window::new("Test", WIDTH, HEIGHT, WindowOptions::default())?;
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    window.limit_update_rate(Some(std::time::Duration::from_micros(1000000)));

    let lower_left_corner = Array1::from(vec![-2., -1., -1.]);
    let horizontal = Array1::from(vec![4., 0., 0.]);
    let vertical = Array1::from(vec![0., 2., 0.]);
    let origin = Array1::from(vec![0., 0., 0.]);

    while window.is_open() && !window.is_key_down(Key::Enter) {
        for (i, bi) in buffer.iter_mut().enumerate() {
            let u = (i % WIDTH) as f64 / WIDTH as f64;
            let v = 1.0 - (i as f64 / WIDTH as f64 / HEIGHT as f64);

            let direction = u * &horizontal + v * &vertical + &lower_left_corner;
            let r = Ray::new(&origin, &direction);
            let col = color(&r);

            let ir = (255.0 * col[0]) as u32;
            let ig = (255.0 * col[1]) as u32;
            let ib = (255.0 * col[2]) as u32;
            *bi = (ir << 16) | (ig << 8) | ib;
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    }
    Ok(())
}
