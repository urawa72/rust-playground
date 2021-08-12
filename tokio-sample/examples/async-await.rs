use std::time::Duration;

use tokio::time::delay_for;

async fn hello() {
    delay_for(Duration::from_millis(1000)).await;
    println!("1 sec elapsed");
}

#[tokio::main]
async fn main() {
    hello().await;

    println!("hello, world");
}
