use metarustistic_functions::rastrigin;
use metarustistic_optimizers::random_walk::RandomWalk;
use metarustistic_plotters::plotter_3d::plot_traj;
use std::time::Instant;
fn main() {
    let mut opti:RandomWalk = RandomWalk{
        positions : vec![0.0, 2.001],
        step_number:1,
    };
    let start = Instant::now();
    let mut min:f64=80.0;
    let mut b_traj:Vec<Vec<f64>>=vec![];
    for _i in 0..1000000{
        opti.step();
        let r = rastrigin(&opti.positions);
        if r<min{
            min = r;
            println!("Best at step {} {}",opti.step_number,min);
            let point:Vec<f64> = vec![opti.positions[0],opti.positions[1],r];
            b_traj.push(point);

        }
    }
    let duration = start.elapsed();
    plot_traj(b_traj);
    println!("Time {:?}", duration)
    
}
