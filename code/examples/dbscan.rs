use code::*;
// ANCHOR: libraries
// Import the linfa prelude and KMeans algorithm
use linfa::prelude::*;
use linfa_clustering::Dbscan;
// We'll build our dataset on our own using ndarray and rand
use ndarray::prelude::*;
// Import the plotters crate to create the scatter plot
use plotters::prelude::*;
// ANCHOR_END: libraries

fn main() {
    // ANCHOR: create_circles
    let circle: Array2<f32> = create_circle([5.0, 5.0], 1.0, 100); // Cluster 0
    let donut_1: Array2<f32> = create_hollow_circle([5.0, 5.0], [2.0, 3.0], 400); // Cluster 1
    let donut_2: Array2<f32> = create_hollow_circle([5.0, 5.0], [4.5, 4.75], 400); // Cluster 2

    let data = ndarray::stack(Axis(0), &[circle.view(), donut_1.view(), donut_2.view()])
        .expect("An error occurred while stacking the dataset");

    // ANCHOR: create_model
    // let dataset = DatasetBase::from(data);
    let min_points = 20;
    let clusters = Dbscan::params(min_points).tolerance(0.95).transform(&data);
    println!("{:#?}", clusters);
    // ANCHOR_END: create_model

    // ANCHOR: build_chart_base
    let root = BitMapBackend::new("../src/dbscan.png", (600, 400)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let x_lim = 0.0..10.0f32;
    let y_lim = 0.0..10.0f32;

    let mut ctx = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40) // Put in some margins
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("DBSCAN Demo", ("sans-serif", 25)) // Set a caption and font
        .build_cartesian_2d(x_lim, y_lim)
        .expect("Couldn't build our ChartBuilder");
    // ANCHOR_END: build_chart_base

    // ANCHOR: configure_chart
    ctx.configure_mesh().draw().unwrap();
    let root_area = ctx.plotting_area();
    // ANCHOR_END: configure_chart

    // ANCHOR: run_check_for_plotting;
    // check_array_for_plotting(dataset: &Array2<f32>) -> bool {}
    check_array_for_plotting(&circle); // Panics if that's not true
                                       // ANCHOR_END: run_check_for_plotting

    // ANCHOR: plot_points
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

        root_area
            .draw(&point)
            .expect("An error occurred while drawing the point!");
    }
    // ANCHOR_END: plot_points
}

// Check the array has the correct shape for plotting (Two-dimensional, with 2 columns)
fn check_array_for_plotting(arr: &Array2<f32>) -> bool {
    if (arr.shape().len() != 2) || (arr.shape()[1] != 2) {
        panic!(
            "Array shape of {:?} is incorrect for 2D plotting!",
            arr.shape()
        );
        // false
    } else {
        true
    }
}
