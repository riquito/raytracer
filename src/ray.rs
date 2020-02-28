use ndarray::Array1;

pub struct Ray {
    pub origin: Array1<f64>,
    pub direction: Array1<f64>,
}

impl Ray {
    pub fn new(origin: Array1<f64>, direction: Array1<f64>) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f64) -> Array1<f64> {
        (t * &self.direction) + &self.origin
    }
}
