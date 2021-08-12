#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async { "hello world".to_string() });

    let result: String = handle.await.unwrap();
    println!("{}", result);
}
