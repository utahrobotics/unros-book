# Hello Goodbye

Hello World is a classic first project, but we also want to showcase how the runtime exits. To start, simply create a new project with any name you'd like:

```shell
cargo new hello-goodbye
```

Once you've done that, simply add the following to the dependencies in `Cargo.toml`:

```toml
unros = "0.1"
```

Now, we shall start coding exclusively in `main.rs`. First, let us modify the `main` method. Here is what a classic one would look like:

```rust
fn main() {
    println!("Hello world!");
}
```

The entire execution of the program is encapsulated within this method. In robotics, we typically have several systems that work concurrently and persistently. There is a lot of boilerplate involved in running these correctly, so we've handled that for you! Simply use the procedural macro, much in the same way as `tokio::main`:

```rust
use unros::{Application, anyhow};

#[unros::main]
async fn main(app: Application) -> anyhow::Result<Application> {
    println!("Hello world!");
    Ok(app)
}
```

We've added quite a few things:

`#[unros::main]`  
Invokes a procedural macro to add the boilerplate code.

`async fn main(app: Application) -> anyhow::Result<Application>`  
`main` can now run asynchronous code. The async runtime is generated automatically. The method now also accepts an `Application` object. Nodes and tasks are added to this `Application` which queues them up for execution. This `Application` object is then returned assuming no errors occurred. If an error did occur, you can safely return it for it to be displayed according to `anyhow`'s formatting.

This `Application` object is key, as it means that the application itself does not run until the `main` method exits. Now, the entire execution of the program is no longer encapsulated in `main`, so what is `main`'s purpose? Initialization. We always want errors to appear as early as possible, so perform all of your extensive error checking here before returning a correctly made `Application` object, and let Unros handle the rest.

So, what does the `Application` do right now? Well, nothing! It has not been modified at all, so Unros will not do anything. Try running the application right now.

```shell
cargo run
```

```log
[0:0.00 unros_core] Runtime started with pid: 220566
Hello World!
[0:0.00 unros_core] All nodes have terminated
[0:0.00 unros_core] Exiting...
```

The program seems to be working just fine! You should also have noticed that a `logs` folder was generated, and there is a `.log` inside with the following contents:

```log
[0:0.00 INFO unros_core] Runtime started with pid: 220566
[0:0.00 DEBUG auxilliary-control] Successfully binded to: 0.0.0.0:44537
[0:0.00 INFO unros_core] All nodes have terminated
[0:0.00 INFO unros_core] Exiting...
```

Note that `Hello World!` is not present. This is because the log file only stores actual logs and not `stdout`. This may be subject to change. Ignore the `auxilliary-control` line for now. Your `pid` will also be different. You can use this to kill the process if all else fails (which hopefully does not happen).
