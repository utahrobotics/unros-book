# Getting Started

Unros is a Rust only framework with no required external dependencies[^deps]. As such, you only need to [install Rust](https://doc.rust-lang.org/book/ch01-01-installation.html).

Unros requires the use of nightly features. The easiest way to do this is to create a `rust-toolchain.toml` with the following contents:

```toml
[toolchain]
channel = "nightly"
```

It also requires the nightly channel for `tokio`, so you should create a folder in your project called `.cargo` with a file inside called `config.toml` with the following contents:

```toml
[build]
rustflags = ["--cfg", "tokio_unstable"]
```

[^deps]: While `ffmpeg` and `ffplay` are used, they are sidecars and are thus not needed until any video encoding or playback is required. When code that requires these is invoked, an error will be returned with `Result` instead of panics, so you will be able to catch those if needed. When possible, `ffmpeg` will be automatically installed if such code is invoked.
