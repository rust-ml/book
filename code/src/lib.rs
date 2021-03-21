use ndarray::prelude::*;
use rand::prelude::*;
use std::f32;

// Creates random points contained in an approximately square area
pub fn create_square(center_point: [f32; 2], edge_length: f32, num_points: usize) -> Array2<f32> {
    // We
    let mut data: Array2<f32> = Array2::zeros((num_points, 2));
    let mut rng = rand::thread_rng();
    for i in 0..num_points {
        let x = rng.gen_range(-edge_length * 0.5, edge_length * 0.5); // generates a float between 0 and 1
        let y = rng.gen_range(-edge_length * 0.5, edge_length * 0.5);
        data[[i, 0]] = center_point[0] + x;
        data[[i, 1]] = center_point[1] + y;
    }

    data
}

// Creates a circle of random points
pub fn create_circle(center_point: [f32; 2], radius: f32, num_points: usize) -> Array2<f32> {
    let mut data: Array2<f32> = Array2::zeros((num_points, 2));
    let mut rng = rand::thread_rng();
    for i in 0..num_points {
        let theta = rng.gen_range(0.0, 2.0 * f32::consts::PI);
        let r = rng.gen_range(0.0, radius);
        let x = r * f32::cos(theta);
        let y = r * f32::sin(theta);

        data[[i, 0]] = center_point[0] + x;
        data[[i, 1]] = center_point[1] + y;
    }

    data
}

// Creates a hollow circle of random points with a specified inner and outer diameter
pub fn create_hollow_circle(
    center_point: [f32; 2],
    radius: [f32; 2],
    num_points: usize,
) -> Array2<f32> {
    assert!(radius[0] < radius[1]);
    let mut data: Array2<f32> = Array2::zeros((num_points, 2));
    let mut rng = rand::thread_rng();
    for i in 0..num_points {
        let theta = rng.gen_range(0.0, 2.0 * f32::consts::PI);
        let r = rng.gen_range(radius[0], radius[1]);
        let x = r * f32::cos(theta);
        let y = r * f32::sin(theta);

        data[[i, 0]] = center_point[0] + x;
        data[[i, 1]] = center_point[1] + y;
    }

    data
}
