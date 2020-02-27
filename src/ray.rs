use ndarray::Array1;

pub struct Ray<'a> {
    pub origin: &'a Array1<f64>,
    pub direction: &'a Array1<f64>,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Array1<f64>, direction: &'a Array1<f64>) -> Ray<'a> {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f64) -> Array1<f64> {
        (t * self.direction) + self.origin
    }
}
