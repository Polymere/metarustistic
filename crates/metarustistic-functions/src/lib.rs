
use std::f64::consts::PI;

pub fn rastrigin(vector: Vec<f64>)->f64{
    let mut r: f64=10.0 * vector.len() as f64;
    vector.iter().for_each(|x: &f64| {
        r+=x*x - 10.0*(2.0*PI*x).cos();
    });
    return r;
}
