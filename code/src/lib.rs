use ndarray::prelude::*;
use rand::prelude::*;

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
