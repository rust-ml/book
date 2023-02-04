use ndarray::prelude::*;
use rand::prelude::*;
use std::f32;

// Creates random points contained in an approximately square area
pub fn create_square(center_point: [f32; 2], edge_length: f32, num_points: usize) -> Array2<f32> {
    // We
    let mut data: Array2<f32> = Array2::zeros((num_points, 2));
    let mut rng = rand::thread_rng();
    for i in 0..num_points {
        let x = rng.gen_range(-edge_length * 0.5..edge_length * 0.5); // generates a float between 0 and 1
        let y = rng.gen_range(-edge_length * 0.5..edge_length * 0.5);
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
        let theta = rng.gen_range(0.0..2.0 * f32::consts::PI);
        let r = rng.gen_range(0.0..radius);
        let x = r * f32::cos(theta);
        let y = r * f32::sin(theta);

        data[[i, 0]] = center_point[0] + x;
        data[[i, 1]] = center_point[1] + y;
    }

    data
}

// Creates a line y = m*x + b with some noise
pub fn create_line(m: f64, b: f64, num_points: usize, min_max: [f64; 2]) -> Array2<f64> {
    let mut data: Array2<f64> = Array2::zeros((num_points, 2));

    let mut rng = rand::thread_rng();
    for i in 0..num_points {
        let var_y = rng.gen_range(-0.5..0.5f64);
        data[[i, 0]] = rng.gen_range(min_max[0]..min_max[1]);
        data[[i, 1]] = (m * data[[i, 0]]) + b + var_y;
    }

    data
}

// Creates a quadratic y = m*x^2 + b with some noise
pub fn create_curve(m: f64, pow: f64, b: f64, num_points: usize, min_max: [f64; 2]) -> Array2<f64> {
    let mut data: Array2<f64> = Array2::zeros((num_points, 2));

    let mut rng = rand::thread_rng();
    for i in 0..num_points {
        let var_y = rng.gen_range(-0.5..0.5f64);
        data[[i, 0]] = rng.gen_range(min_max[0]..min_max[1]);
        data[[i, 1]] = (m * data[[i, 0]].powf(pow)) + b + var_y;
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
        let theta = rng.gen_range(0.0..2.0 * f32::consts::PI);
        let r = rng.gen_range(radius[0]..radius[1]);
        let x = r * f32::cos(theta);
        let y = r * f32::sin(theta);

        data[[i, 0]] = center_point[0] + x;
        data[[i, 1]] = center_point[1] + y;
    }

    data
}

// Check the array has the correct shape for plotting (Two-dimensional, with 2 columns)
pub fn check_array_for_plotting(arr: &Array2<f32>) -> bool {
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
