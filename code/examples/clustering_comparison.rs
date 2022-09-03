use linfa_book::*;
// ANCHOR: libraries
// Import the linfa prelude and KMeans algorithm
use linfa::prelude::*;
use linfa_clustering::{Dbscan, KMeans};
// We'll build our dataset on our own using ndarray and rand
use ndarray::prelude::*;
// Import the plotters crate to create the scatter plot
use plotters::prelude::*;
use rand::prelude::*;
// ANCHOR_END: libraries

fn main() {
    // ANCHOR: build_chart_base
    let chart_dims = (900, 400);
    let root =
        BitMapBackend::new("../src/clustering_comparison.png", chart_dims).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let areas = root.split_by_breakpoints([chart_dims.0 / 2], [chart_dims.1]);

    let x_lim = 0.0..10.0f32;
    let y_lim = 0.0..10.0f32;

    let mut ctx_a = ChartBuilder::on(&areas[0])
        .set_label_area_size(LabelAreaPosition::Left, 40) // Put in some margins
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("DBSCAN", ("sans-serif", 20)) // Set a caption and font
        .build_cartesian_2d(x_lim.clone(), y_lim.clone())
        .expect("Couldn't build our ChartBuilder");

    ctx_a.configure_mesh().draw().unwrap();
    let root_area_a = ctx_a.plotting_area();

    let mut ctx_b = ChartBuilder::on(&areas[1])
        .set_label_area_size(LabelAreaPosition::Left, 40) // Put in some margins
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("KMeans", ("sans-serif", 20)) // Set a caption and font
        .build_cartesian_2d(x_lim, y_lim)
        .expect("Couldn't build our ChartBuilder");

    ctx_b.configure_mesh().draw().unwrap();
    let root_area_b = ctx_b.plotting_area();

    // Plot space for both algos should be set up by this point

    // Now going to creat data that will be shared for both
    let circle: Array2<f32> = create_circle([5.0, 5.0], 1.0, 100); // Cluster 0
    let donut_1: Array2<f32> = create_hollow_circle([5.0, 5.0], [2.0, 3.0], 400); // Cluster 1
    let donut_2: Array2<f32> = create_hollow_circle([5.0, 5.0], [4.5, 4.75], 1000); // Cluster 2
    let noise: Array2<f32> = create_square([5.0, 5.0], 10.0, 100); // Random noise

    let data = ndarray::concatenate(
        Axis(0),
        &[circle.view(), donut_1.view(), donut_2.view(), noise.view()],
    )
    .expect("An error occurred while stacking the dataset");

    // DBSCAN STARTS HERE

    let min_points = 20;
    let clusters = Dbscan::params(min_points)
        .tolerance(0.6)
        .transform(&data)
        .unwrap();
    println!("{:#?}", clusters);

    check_array_for_plotting(&circle); // Panics if that's not true

    for i in 0..data.shape()[0] {
        let coordinates = data.slice(s![i, 0..2]);

        let point = match clusters[i] {
            Some(0) => Circle::new(
                (coordinates[0], coordinates[1]),
                3,
                ShapeStyle::from(&RED).filled(),
            ),
            Some(1) => Circle::new(
                (coordinates[0], coordinates[1]),
                3,
                ShapeStyle::from(&GREEN).filled(),
            ),
            Some(2) => Circle::new(
                (coordinates[0], coordinates[1]),
                3,
                ShapeStyle::from(&BLUE).filled(),
            ),
            // Making sure our pattern-matching is exhaustive
            _ => Circle::new(
                (coordinates[0], coordinates[1]),
                3,
                ShapeStyle::from(&BLACK).filled(),
            ),
        };

        root_area_a
            .draw(&point)
            .expect("An error occurred while drawing the point!");
    }
    // ANCHOR_END: plot_points

    // KMEANS STARTS HERE

    let dataset = DatasetBase::from(data);
    let rng = thread_rng(); // Random number generator
    let n_clusters = 3;
    let model = KMeans::params_with_rng(n_clusters, rng)
        .max_n_iterations(200)
        .tolerance(1e-5)
        .fit(&dataset)
        .expect("Error while fitting KMeans to the dataset");

    let dataset = model.predict(dataset);

    for i in 0..dataset.records.shape()[0] {
        let coordinates = dataset.records.slice(s![i, 0..2]);

        let point = match dataset.targets[i] {
            0 => Circle::new(
                (coordinates[0], coordinates[1]),
                3,
                ShapeStyle::from(&RED).filled(),
            ),
            1 => Circle::new(
                (coordinates[0], coordinates[1]),
                3,
                ShapeStyle::from(&GREEN).filled(),
            ),

            2 => Circle::new(
                (coordinates[0], coordinates[1]),
                3,
                ShapeStyle::from(&BLUE).filled(),
            ),
            // Making sure our pattern-matching is exhaustive
            _ => Circle::new(
                (coordinates[0], coordinates[1]),
                3,
                ShapeStyle::from(&BLACK).filled(),
            ),
        };

        root_area_b
            .draw(&point)
            .expect("An error occurred while drawing the point!");
    }
}
