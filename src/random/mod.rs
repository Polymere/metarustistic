use rand::Rng;
pub fn get_random_vec(n: i32, low: f32, high: f32) -> Vec<f32> {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let vals: Vec<f32> = (0..n).map(|_| rng.gen_range(low..=high)).collect();
    return vals;
}