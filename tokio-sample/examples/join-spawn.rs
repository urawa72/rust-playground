#[tokio::main]
async fn main() {
    let fut1 = async { 1 };
    let fut2 = async { "hello".to_string() };

    let handle1 = tokio::spawn(fut1);
    let handle2 = tokio::spawn(fut2);

    if let (Ok(res1), Ok(res2)) = tokio::join!(handle1, handle2) {
        println!("{}, {}", res1, res2);
    }
}
