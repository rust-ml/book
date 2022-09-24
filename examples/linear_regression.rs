use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::prelude::*;
use plotters::prelude::*;

fn main() {
    let array: Array2<f64> = linfa_book::create_line(1.0, 2.0, 50, [0., 7.]).mapv(|x| x as f64);

    // Converting from an array to a Linfa Dataset can be the trickiest part of this process
    let (data, targets) = (
        array.slice(s![.., 0..1]).to_owned(),
        array.column(1).to_owned(),
    );

    let dataset = Dataset::new(data, targets).with_feature_names(vec!["x", "y"]);

    let lin_reg = LinearRegression::new();
    let model = lin_reg.fit(&dataset).unwrap();

    let root_area = BitMapBackend::new("plots/linear.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Legend", ("sans-serif", 40))
        .caption("Linear Regression", ("sans-serif", 40))
        .build_cartesian_2d(0.0..8f64, 0.0..10f64)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let mut line_points = Vec::with_capacity(2);
    for i in (0..8i32).step_by(1) {
        line_points.push((i as f64, (i as f64 * model.params()[0]) + model.intercept()));
    }
    // We can configure the rounded precision of our result here
    let precision = 2;
    let label = format!(
        "y = {:.2$}x + {:.2}",
        model.params()[0],
        model.intercept(),
        precision
    );
    ctx.draw_series(LineSeries::new(line_points, &BLACK))
        .unwrap()
        .label(&label);

    let num_points = array.shape()[0];
    let mut points = Vec::with_capacity(num_points);
    for i in 0..array.shape()[0] {
        let point = (array[[i, 0]], array[[i, 1]]);
        let circle = Circle::new(point, 5, &RED);
        points.push(circle);
    }

    ctx.draw_series(points).unwrap();

    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .draw()
        .unwrap();
}
