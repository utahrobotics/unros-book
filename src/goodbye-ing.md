# Goodbye-ing

Now, how do we actually do the goodbye part since when `main` exits, there is still additional code that runs. How about implementing `Drop`?

```rust
use unros::{Application, anyhow};


struct Goodbyer;

impl Drop for Goodbyer {
    fn drop(&mut self) {
        println!("Goodbye World!");
    }
}


#[unros::main]
async fn main(app: Application) -> anyhow::Result<Application> {
    println!("Hello world!");
    let _goodbye = Goodbyer;
    Ok(app)
}
```

Now try running this.

```log
[0:0.00 INFO unros_core] Runtime started with pid: 220566
Hello World!
Goodbye World!
[0:0.00 INFO unros_core] All nodes have terminated
[0:0.00 INFO unros_core] Exiting...
```

Alright, we've finished what we've started! However, Unros still has not done anything for us. Namely, it has not ran any Nodes yet. Let's turn `Goodbyer` into a node!

```rust
use unros::{Application, anyhow, async_trait, Node, NodeIntrinsics, RuntimeContext};


#[derive(Default)]
struct Goodbyer {
    intrinsics: NodeIntrinsics<Self>
}

impl Drop for Goodbyer {
    fn drop(&mut self) {
        println!("Goodbye World!");
    }
}

#[async_trait]
impl Node for Goodbyer {
    const DEFAULT_NAME: &'static str = "goodbye";
    
    async fn run(self, _context: RuntimeContext) -> anyhow::Result<()> {
        Ok(())
    }

    fn get_intrinsics(&mut self) -> &mut NodeIntrinsics<Self> {
        &mut self.intrinsics
    }
}


#[unros::main]
async fn main(mut app: Application) -> anyhow::Result<Application> {
    println!("Hello world!");
    app.add_node(Goodbyer::default());
    Ok(app)
}
```

Now this is a big jump, so lets go over it step by step.

`NodeIntrinsics<Self>`  
This is an object that helps `unros` track the state of a node as the app runs. It even tells you if you've made a node but have not added it to the app yet! Try commenting out the line with `add_node` and run.

`#[async_trait]`  
`Node` is actually a trait with async functions, so until Rust adds that functionality natively with all the appropriate thread safety stuff, we are stuck with this approach.

`const DEFAULT_NAME: &'static str = "goodbye";`  
For better logging, nodes should have names. However, having to provide a name every time you add a node is annoying, so nodes can have default names that are used when a name is not provided.

`async fn run(self, _context: RuntimeContext) -> anyhow::Result<()>`  
This is the main body of the node. A node is only ran once, so everything that a node needs to do should be in this method. For now, we just return `Ok`.

`fn get_intrinsics(&mut self) -> &mut NodeIntrinsics<Self>`  
This method is called by `unros` so don't worry about it. All you need to do is return a mutable reference to the field containing your intrinsics (you should only ever have 1).

If you run this, you will get the same output, so why don't we make the node wait for a few seconds before saying goodbye?

```rust
#[async_trait]
impl Node for Goodbyer {
    const DEFAULT_NAME: &'static str = "goodbye";
    
    async fn run(self, _context: RuntimeContext) -> anyhow::Result<()> {
        unros::tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        Ok(())
    }

    fn get_intrinsics(&mut self) -> &mut NodeIntrinsics<Self> {
        &mut self.intrinsics
    }
}
```

Now, `Goodbyer` will wait for three seconds before exiting. Since it is the only node, when it exits, the entire runtime also exits. Note that we are still printing `Goodbye World!`, so the `Drop` implementation is still being used. This is an important feature of Unros. All Nodes will be dropped, *even* if forcefully exiting by pressing `Ctrl-C` twice!

You have officially written your first Node! Theoretically, you could publish this crate as a library, allowing people to add this Node to their applications just by calling `add_node`. In ROS, Node's offered a trivial way to perform multithreading and error resistance. Rust has a much better system for concurrency, so Nodes themselves could be multithreaded. Instead, Nodes in Unros are a form of encapsulation that allow better code reuse and sharing, whilst also providing more useful and relevant logging. Always prefer to use Nodes over just spawning tasks and threads!
