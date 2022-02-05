use std::process::exit;
use tokio::io::{stdout, AsyncWriteExt};

#[tokio::main]
async fn main() {
    stdout().write("Hello, world!\n".as_bytes()).await.unwrap();

    exit(0);
}
