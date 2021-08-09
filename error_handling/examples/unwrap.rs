fn main() {
    let result: Result<i32, String> = Err("error".to_string());
    println!("{:?}", result.unwrap_or(10));

    let result: Result<i32, String> = Err("error".to_string());
    println!("{:?}", result.unwrap_or_else(|_| 20));

    let option: Option<i32> = None;
    println!("{:?}", option.unwrap_or(100));

    let option: Option<i32> = None;
    println!("{:?}", option.unwrap_or_else(|| 200));
}
