## Using DBSCAN with `linfa-clustering`

### What is the DBSCAN algorithm?

The DBSCAN algorithm (Density-Based Spatial Clustering Algorithm with Noise) was originally published in [1996](https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.121.9220), and has since become one of the most popular and well-known clustering algorithms available. 


### Comparison with KMeans

Before getting into the code, let's examine how these differences in approach plays out on different types of data. In the images below, both the DBSCAN and KMeans algorithms were applied to the same dataset. The KMeans algorithm was manually set to find 3 clusters (remember, DBSCAN automatically calculates the number of clusters based on the provided parameters).

<img src="assets/clustering_comparison.png" alt="Comparison" width=600px height=300px align="middle">

This example[^1] demonstrates two of the major strengths of DBSCAN over an algorithm like KMeans; it is able to automatically detect the number of clusters that meet the set of given parameters. Keep in mind that this doesn't mean DBSCAN require less information about the dataset, but rather that the information it does require differs from an algorithm like KMeans.

DBSCAN does a great job at finding clustering that are spatially contiguous, but not necessarily confined to single region. This is where the "and Noise" part of the algorithm's name comes in. Especially in real-world data, there's often data that won't fit well into a given cluster. These can be outliers or points that don't demonstrate good alignment with any of the main clusters. DBSCAN doesn't require that they do. Instead, it will simply give them a cluster label of `None` (in our example, these are graphically the black points). However, DBSCAN does a good job at analyzing existing information, it doesn't predict new data, which is one of its main drawbacks

Comparatively, KMeans will take into account each point in the dataset, which means outliers can negatively affect the local optimal location for a given cluster's centroid in order to accommodate them. Euclidean space is linear, which means that small changes in the data result in proportionately small changes to the position of the centroids. This is problematic when there are outliers in the data.

### Using DSBCAN with `linfa`

Compared to 
```rust,no_run
{{#include ../examples/dbscan.rs:libraries}}
```

Instead of having a several higher-density clusters different areas, we'll take advantage of DBSCAN's ability to follow spatially-contiguous non-localized clusters by building our data out of both filled and hollow circles, with some random noise tossed in as well. The end goal will be to re-find each of these clusters, and exclude some of the noise!

```rust,no_run
{{#include ../examples/dbscan.rs:create_circles}}
```
Compared to `linfa`'s KMeans algorithm, the DBSCAN implementation is able to operate directly on a ndarray `Array2` data structure, so there's no need to convert it into the `linfa`-native `Dataset` type first. It's also worth pointing out that choosing the chosen parameters often take some experimentation and tuning before they produce results that actually make sense. This is one of the areas where data visualization can be really valuable; it is helpful in developing some spatial intuition about your data set and understand how your choice of hyperparameters will affect the results produced by the algorithm.  

```rust,no_run
{{#include ../examples/dbscan.rs:create_and_run_model}}
```
We'll skip over setting up `ChartBuilder` struct and drawing areas from the `plotters` crate, since it's exactly the same as in the [KMeans](./3_kmeans.md) example. 

Remember how we mentioned DBSCAN is an algorithm that can exclude noise? That's particularly important for the pattern-matching in this case, since we're almost guaranteed to end up with some values that don't fit nicely into any of our expected clusters. Since we generated an artificial dataset, we know the number of clusters that should be generated, and where they're located. However, that won't always be the case. In that situation, we could instead examine the number of clusters afterwards, create a colormap using custom RGB colors which matches the highest number of clusters, and plot it that way.

```rust,no_run
{{#include ../examples/dbscan.rs:plot_points}}
```

As a result, we then get the following chart, where each cluster is uniquely identified, and some of the random noise associated with the dataset is discarded.  

<img src="assets/dbscan.png" alt="DBSCAN" width=500px height=450px align="middle">

---
[^1]: This code for this comparison is actually separate from the main DBSCAN example. It can be found at `examples/clustering_comparison.rs`.