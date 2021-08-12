#[tokio::main]
async fn main() {
    let fut1 = async { 1 };
    let fut2 = async { "hello".to_string() };

    let (res1, res2): (i32, String) = tokio::join!(fut1, fut2);

    println!("{}, {}", res1, res2);
}
