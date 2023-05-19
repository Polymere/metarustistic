use metarustistic_functions::rastrigin;
fn main() {
    let vec = vec![0.0, 2.001, 0.0];
    let rastrigin: f64 = rastrigin(vec.clone());
    println!("Vector {:?}", vec);
    println!("Rastrigin {}", rastrigin);
}
