fn main() {
    let result: Result<i32, String> = Ok(2);
    println!("{:?}", result.unwrap());

    // should check not Err
    let result: Result<i32, String> = Err("error".to_string());
    println!("{:?}", result.unwrap());
}
