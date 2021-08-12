async fn hello() -> String {
    "hello, async fn".to_string()
}

#[tokio::main]
async fn main() {
    let greeting: String = hello().await;
    println!("{}", greeting);

    let world = async {
        println!("hello, async block");
    };
    world.await;
}
