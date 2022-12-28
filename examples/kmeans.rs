use linfa_book::*;
// ANCHOR: libraries
// Import the linfa prelude and KMeans algorithm
use linfa::prelude::*;
use linfa_clustering::KMeans;
use linfa_nn::distance::LInfDist;
// We'll build our dataset on our own using ndarray and rand
use ndarray::prelude::*;
use rand::prelude::*;
// Import the plotters crate to create the scatter plot
use plotters::prelude::*;
// ANCHOR_END: libraries

fn main() {
    // ANCHOR: create_squares
    let square_1: Array2<f32> = create_square([7.0, 5.0], 1.0, 150); // Cluster 1
    let square_2: Array2<f32> = create_square([2.0, 2.0], 2.0, 150); // Cluster 2
    let square_3: Array2<f32> = create_square([3.0, 8.0], 1.0, 150); // Cluster 3
    let square_4: Array2<f32> = create_square([5.0, 5.0], 9.0, 300); // A bunch of noise across them all

    let data: Array2<f32> = ndarray::concatenate(
        Axis(0),
        &[
            square_1.view(),
            square_2.view(),
            square_3.view(),
            square_4.view(),
        ],
    )
    .expect("An error occurred while stacking the dataset");
    //ANCHOR_END: create_squares

    // ANCHOR: create_model
    let dataset = DatasetBase::from(data);
    let rng = thread_rng(); // Random number generator
    let n_clusters = 3;
    let model = KMeans::params_with(n_clusters, rng, LInfDist)
        .max_n_iterations(200)
        .tolerance(1e-5)
        .fit(&dataset)
        .expect("Error while fitting KMeans to the dataset");
    // ANCHOR_END: create_model

    // ANCHOR: run_model
    let dataset = model.predict(dataset);
    println!("{:?}", dataset.records.shape());
    println!("{:?}", dataset.targets.shape());
    // ANCHOR_END: run_model

    // ANCHOR: build_chart_base
    let root = BitMapBackend::new("target/kmeans.png", (600, 400)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let x_lim = 0.0..10.0f32;
    let y_lim = 0.0..10.0f32;

    let mut ctx = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40) // Put in some margins
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("KMeans Demo", ("sans-serif", 25)) // Set a caption and font
        .build_cartesian_2d(x_lim, y_lim)
        .expect("Couldn't build our ChartBuilder");
    // ANCHOR_END: build_chart_base

    // ANCHOR: configure_chart
    ctx.configure_mesh().draw().unwrap();
    let root_area = ctx.plotting_area();
    // ANCHOR_END: configure_chart

    // ANCHOR: run_check_for_plotting;
    // check_array_for_plotting(dataset: &Array2<f32>) -> bool {}
    check_array_for_plotting(&dataset.records); // Panics if that's not true
                                                // ANCHOR_END: run_check_for_plotting

    // ANCHOR: plot_points
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

        root_area
            .draw(&point)
            .expect("An error occurred while drawing the point!");
    }
    // ANCHOR_END: plot_points
}
