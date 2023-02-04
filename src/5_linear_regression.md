## Linear Regression

Now that we've gotten some clustering under our belt, let's take a look at one of the other common data science tasks: linear regression on two-dimensional data. This example includes code for both calculating the linear equation using `linfa`, as well as code for plotting both the data and line on a single graph using the `plotters` library. 

Per usual, we'll create some data using one of our built-in functions. This simply creates an `Array2<f64>` with two columns, one of which will be our x-axis and the other our y-axis. We're generating this artificially, but remember, we could get this from a real data source like processing a CSV file or reading in values from a sensor.

```rust,no_run
{{#include ../examples/linear_regression.rs:create_data}}
```
Now that we have the initial data, let's break that down into something that we can use for our regression; a `data` array and a `target` array. Fortunately, this is pretty simple with the `slice()` and `column()` functions provided by `ndarray`. We're also going to want to grab the maximum values for our arrays (and round them up to the nearest integer using the `ceil()` function) to be used for plotting those values a little bit later. 

```rust,no_run
{{#include ../examples/linear_regression.rs:data_format}}
```

Once the data is formatted, we'll be able to nicely add it into the `linfa`-native `Dataset` format, along with the appropriate feature names. If you're running into funky error related to array shapes in your code, this section and the step before (where we create our `data` and `target` data structures) are ones you should double-check; dynamically-shaped arrays as found in most scientific computing libraries, Rust-based or not, can be tricky. 

In fact, as you may have experienced yourself, it's very common that the pre-processing steps of many data science problems (filtering, formatting, distributing, etc.) are actually the most complicated and often where a little bit of additional effort can save you a lot of trouble down the road.

```rust,no_run
{{#include ../examples/linear_regression.rs:build_dataset}}
```

However, now we have our data formatted properly and in the `Dataset` format, actually running the regression is pretty simple; we only need to create our `LinearRegression` object and fit it to the dataset. 

```rust,no_run
{{#include ../examples/linear_regression.rs:regression}}
```

We're going to leave out a little bit of the boilerplate (check the repository for the full example code), but you'll notice that when we set up our chart context, we'll use the rounded maximum values in both the `data` and `target` arrays to set our maximum chart range (as mentioned earlier).

```rust,no_run
{{#include ../examples/linear_regression.rs:chart_context}}
```

Now that the chart is good to go, we'll start off by drawing our best fit line using the linear equation we derived above. We can't just supply the equation and let the plotting figure it out; instead, what we'll do it create series of points that exactly match this equation at regular intervals, and connect those with a smooth, continuous line. If this seems clunky, just remember: we have a nice, smooth solution this time around, but that might not always be the case. In the future, we might want more complicated polynomial, or even a discontinuous function. This approach (smoothly connecting an arbitrary set of points) is applicable to a wide variety of potential applications.

Once we add our line, we'll also want a nice label, with a set level of precision; this will be added to the legend once our chart is complete.

```rust,no_run
{{#include ../examples/linear_regression.rs:draw_line}}
```

Now that the line is present, we can add our points; this should look very familiar, as we're functionally doing something similar to the clustering examples we've already put together. 

```rust,no_run
{{#include ../examples/linear_regression.rs:draw_points}}
```

Finally, we'll configure the labels that we'll assigned to each of the series that we've drawn on the chart.

```rust,no_run
{{#include ../examples/linear_regression.rs:labels}}
```

And we're done (ooooh, ahhhh, pretty)!

<img src="assets/linear_regression.png" alt="linear regression" width=600px height=400px align="middle">
