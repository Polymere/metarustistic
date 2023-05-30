
use std::f32::consts::PI;

pub fn rastrigin(vector: &Vec<f32>) -> f32 {
    let mut r: f32 = 10.0 * vector.len() as f32;
    vector.iter().for_each(|x: &f32| {
        r += x.powf(2.0) - 10.0 * (2.0 * PI * x).cos();
    });
    return r as f32;
}
