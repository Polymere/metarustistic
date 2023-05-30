use plotters::prelude::*;
use metarustistic_functions::rastrigin;
pub fn plot_traj(values:Vec<Vec<f32>>){
    let root = BitMapBackend::new("/home/paul/3d-line.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .caption("3D Line", ("sans-serif", 40))
        .build_cartesian_3d(-5.12..5.12 as f64, 0.0..10.0 as f64, -5.12..5.12 as f64)
        .unwrap();
    chart.with_projection(|mut pb| {
        pb.yaw = 0.5;
        pb.scale = 0.9;
        pb.into_matrix()
    });
    chart.configure_axes().draw().unwrap();
    chart
        .draw_series(
            SurfaceSeries::xoz(
                (-51..51).map(|f| f as f64/10.0),
                (-51..51).map(|f| f as f64/10.0),
                |x,z| rastrigin(&vec![x as f32,z as f32])as f64,
            )
            .style(BLUE.mix(0.1).filled()),
        ).unwrap();
    chart.draw_series(LineSeries::new(
        values.iter().map(|p| (p[0] as f64,p[2]as f64,p[1]as f64)),
        &RED
    )).unwrap();
}