use plotters::prelude::*;
pub fn plot_traj(values:Vec<Vec<f64>>){
    let root = BitMapBackend::new("/home/paul/3d-line.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("3D Line", ("sans-serif", 40))
        .build_cartesian_3d(-5.12..5.12 as f64, 0.0..20.0 as f64, -5.12..5.12 as f64)
        .unwrap();
    chart.configure_axes().draw().unwrap();

    // let x_iter= values.iter().map(|p| p[0]).collect();
    // let y_iter = values.iter().map(|p| p[1]).collect();
    // let z_iter= values.iter().map(|p|p[2]).collect();
    chart.draw_series(LineSeries::new(
        values.iter().map(|p| (p[0],p[1],p[2])),
        &RED
    )).unwrap();
}