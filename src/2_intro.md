# Introduction to Machine Learning with Rust

TO DO:
- Description of the Rust-ML group
- Introduction to the ecosystem
    - ndarray common linear algebra backend for scientific computing
    - linfa is the Rust-ML group's meta-crate for classical machine learning and data science, analogous to Python's scikit-learn
    - The data visualization is generally provided by the `plotters` crate
- Reminder about ethical use in machine learning and paying attention to how your work is used
    - https://github.com/EthicalML/awesome-artificial-intelligence-guidelines

## How To Use This Book
This book aims to provide an introduction to the commonly used algorithms used for machine learning and data science in the Rust ecosystem. Each chapter will have a description of an algorithm, and walk through a code example from start to finish.

Each of those code examples are located in the `code/` directory, and can be run independently of the book. For example, to run the entirety of the code example for the KMeans algorithm, the steps would look like the following:
```bash
user@computer:~/rust-ml/book$ cd code/
user@computer:~/rust-ml/book/code$ cargo run --release --example kmeans
```