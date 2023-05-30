
use metarustistic::problem::OptimizationProblem;
use metarustistic::functions::rastrigin;
use metarustistic::optimizers::random_walk::RandomWalk;

fn main(){
    let problem: OptimizationProblem = OptimizationProblem {
        function: BoundedFunction {
            function: rastrigin,
            min_bound: -5.12,
            max_bound: 5.12,
            n_dim: 2,
        },
        optimizer: RandomWalk = Default::default()
    };
}
// fn main() {
//     let start = Instant::now();
//     let mut opti: RandomWalk = RandomWalk {
//         positions: vec![0.0, 2.001],
//         step_number: 1,
//     };
//     let mut min: f64 = 80.0;
//     let mut b_traj: Vec<Vec<f32>> = vec![];
//     for i in 0..1000000 {
//         opti.step();
//         let r = rastrigin(&opti.positions);
//         if r < min {
//             min = r;
//             println!("Best at step {} {}", opti.step_number, min);
//             let point: Vec<f32> = vec![opti.positions[0], opti.positions[1], r as f32];
//             b_traj.push(point);
//         }
//     }
//     let duration = start.elapsed();
//     println!("Time {:?}", duration)
// }


