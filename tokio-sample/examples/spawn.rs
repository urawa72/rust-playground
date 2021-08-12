use std::time::Duration;

use tokio::time::delay_for;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        delay_for(Duration::from_millis(1000)).await;
        println!("1 sec elapsed");
    });

    println!("hello, world");

    // hack: wait until async block finished
    delay_for(Duration::from_millis(2000)).await;
}
