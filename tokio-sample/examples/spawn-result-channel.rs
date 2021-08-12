use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async {
        tx.send("hello, world".to_string()).unwrap();
    });

    let result = rx.await.unwrap();
    println!("{}", result);
}
