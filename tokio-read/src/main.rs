use std::process::exit;
use tokio::io::{stdout, AsyncWriteExt};

#[tokio::main]
async fn main() {
    stdin().read("Hello, world!".as_bytes()).await.unwrap();

    exit(0);
}
