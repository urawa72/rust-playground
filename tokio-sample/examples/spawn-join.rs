use std::time::Duration;

use tokio::time::delay_for;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        delay_for(Duration::from_millis(1000)).await;
        println!("1 sec elapsed");
    });

    println!("hello, world");

    // not hack: wait until async block finished
    let _ = handle.await.unwrap();
}
