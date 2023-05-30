use metarustistic_functions::rastrigin;
use metarustistic_optimizers::pso::PSO;
use metarustistic_plotters::plotter_3d::plot_traj;
use std::time::Instant;
fn main() {
    let mut opti: PSO = PSO {
        n_dim: 2,
        n_particles: 10,
        ..Default::default()
    };
    let start = Instant::now();
    let mut min: f32 = 80.0;
    let mut b_traj: Vec<Vec<f32>> = vec![];
    for _i in 0..1000000 {
        opti.step(rastrigin);
        // let r = rastrigin(&opti.positions);
        if let Some(ref best_particle)=opti.best_particle
        {
            // min = r;
            println!("Best at step {} {}", opti.step_number, min);
            let point: Vec<f32> = vec![best_particle[0], best_particle[1], min];
            b_traj.push(point);
        }
    }
    let duration = start.elapsed();
    plot_traj(b_traj);
    println!("Time {:?}", duration)
}
