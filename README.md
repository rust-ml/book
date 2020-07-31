# The Rust Machine Learning Book

This repository contains the source of "The Rust Machine Learning Book".

## Purpose

The aim of this book is to demonstrate how the Rust language can be used for Machine Learning tasks. They encompass classical ML algorithms, like `Naive Bayes Classifier`, but also more modern approaches. Most of the classical algorithms are contained in the `rust-ml/linfa` crate and ready to use.

## Audience

The reader should have a basic knowledge of Rust type-system and linear algebra. A small recap on `rust-ndarray` type system should familiarize the reader with its applications and limitations.

## Requirements

Building this book requires [mdBook](https://github.com/rust-lang/mdBook) and the [scientific](https://github.com/bytesnake/mdbook-scientific) plugin.
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
```

in order to open it afterwards.
