# Introduction

**Unros** is a fully Rust framework targeted towards robotics applications. It attempts to follow in the footsteps of the Robotic Operating System 1 & 2 while leveraging Rust's unique features.

This framework aims to be memory safe, promote correctness, opinionated, and require minimal upfront configuration and setup. Developed in tandem with the framework is an entire ecosystem that provide various functionality like networking, serial connection, kinematics, autonomy, etc. In this regard, we follow Rust's ideology of delegating extra functionality to separate crates.

## Features

There are several important things define **Unros** and its purpose:

1. **Nodes** - An analogue to ROS Nodes. Since Rust already has *fearless* concurrency, Nodes simply encourage developers to write portable code with distinct functionality.
2. **Async Runtime** - Unros adopts an async first runtime powered by `tokio`. Even if your code is not asynchronous, async runtimes are easier to manage and gracefully shutdown.
3. **PubSub** - A direct analogue to ROS publisher and subscribers. These are akin to mpmc channels except that each value is cloned for every consumer. Their API also attempts to mirror Rust's iterators for added flexibility.
4. **Logging** - Unros has a complete logging solution in the form of text *and* video. Indeed, Unros can use existing `ffmpeg` installations (or automatically install one) to record and play videos on the spot from any source of images.
5. **Integration** - Unros uses common crates to better integrate with the Rust ecosystem as a whole. These include `nalgebra`, `image`, `log`, and `serde`. Re-inventing the wheel is not a goal!

## The Book

This book intends to provide a guided, hands-on tour of Unros and several useful crates of the ecosystem. By the end of this book, you should be able to understand the philosophy of this crate enough to write your own libraries and applications using this framework. You are expected to have an intermediate understanding of Rust. Understanding up to [Chapter 10](https://doc.rust-lang.org/book/ch10-00-generics.html) of the Rust Book is good, but getting up to [Chapter 17](https://doc.rust-lang.org/book/ch17-00-oop.html) is much better.
