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
        unros::tokio::time::sleep(std::time::Duration::from_secs(3)).await;
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