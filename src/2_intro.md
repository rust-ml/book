# Introduction to Machine Learning with Rust

## What is this book for?

This book aims to provide an accessible introduction to machine learning and data science in the Rust ecosystem. Each chapter will have the description of an algorithm, and walk through a code example from start to finish.

## Who is this book for?

This book is written with two primary audiences in mind: developers who are familiar with machine learning and want to write their code Rust, and developers who are familiar with Rust and want to do some machine learning. 

In both cases, we generally assume a basic level of understanding of the Rust programming language, although mastery is certainly not required! If you're brand new to the language, it's suggested to start off by reading [*The Rust Programming Language*](https://doc.rust-lang.org/book/), then returning when you feel a little more comfortable. In particular, it's worth reviewing the sections on [ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html), [error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html), and [functional features](https://doc.rust-lang.org/book/ch13-00-functional-features.html). Perhaps just as importantly as Rust's syntax, a familiarity with the library/crates ecosystem and documentation practices will prove very valuable. Machine learning in many cases sits near the top of the stack; especially when one is working with data, there are usually several layers of code beneath what the top one is doing. That's one of the benefits of working in Rust; these lower layers are often also written in Rust, which makes the abstraction more transparent and empowers developers to dig fearlessly into the underlying aspects of these programs.

Conversely, we don't assume an in-depth knowledge of machine learning (i.e. mathematical familiarity of the field). Some familiarity with the algorithms may be helpful, but the descriptions and code contained in here should help to build a foundation of some of these topics.  

## How to use this book

Each chapter's code sample will be available in its entirety in the `code/` directory, and can be run independently of the book. For example, to run the entirety of the code example for the KMeans algorithm, the steps would look like the following:
```bash
user@computer:~/rust-ml/book cd code/
user@computer:~/rust-ml/book/code cargo run --release --example kmeans
```

## An additional note

Like much of Rust, many of the libraries in this ecosystem empower people to write code that they might otherwise not feel able to write otherwise. Machine learning provides a really interesting and useful set of tools. That is a great benefit! However, as the saying goes, with great power comes great responsibility. This means that **it is the responsibility of each developer individually, and the community as a whole, to make sure that the code we write is not being used in harmful ways and make ethical decisions surrounding our work.**

As a start, we suggest making yourself familiar with some of the resources that have been collected by the Institute for Ethical Machine Learning [here](https://github.com/EthicalML/awesome-artificial-intelligence-guidelines). 

