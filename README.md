# The Rust Machine Learning Book

This repository contains the source of "The Rust Machine Learning Book".

## Purpose

The aim of this book is to demonstrate how the Rust language can be used for Machine Learning tasks. They encompass classical ML algorithms, like linear regression and KMeans clustering, but also more modern approaches. Most of the classical algorithms are contained in the `rust-ml/linfa` crate and ready to use.

## Audience

The reader should have a basic knowledge of Rust type-system and linear algebra. A small recap on `rust-ndarray` type system should familiarize the reader with its applications and limitations.

## Requirements

Building this book requires [mdBook](https://github.com/rust-lang/mdBook). 
```bash
$ cargo install mdbook
```

## Building
You can build the book with 

```bash
$ mdbook build
```

and append

```bash
$ mdbook build --open
# 
$ mdbook serve
```

in order to open it afterwards.

Code samples are contained in the `examples/` directory, and can be built as a group or individually using:
```bash
$ cargo build --all
# or
$ cargo run --example name_of_algorithm
```

By default, all plots will be written to the `target/` directory, so as not to be indexed by git. 