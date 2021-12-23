use std::str;
use tokio::{fs::File, io::AsyncReadExt};

#[tokio::main]
async fn main() {
    let mut buffer = vec![0; 6];

    File::open("./README.md")
        .await
        .unwrap()
        .read(&mut buffer)
        .await
        .unwrap();

    println!("{}", str::from_utf8(&buffer).unwrap());
}
