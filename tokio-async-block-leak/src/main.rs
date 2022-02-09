use std::sync::Arc;
use tokio::runtime::Runtime;

fn main() {
    Runtime::new()
        .unwrap()
        .block_on(async { run(42.into()).await });
}

async fn run(x: Arc<usize>) -> usize {
    println!("{}", x);

    *x
}
