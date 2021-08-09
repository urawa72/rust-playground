fn main() {
    let option: Option<i32> = Some(2);
    println!("{:?}", option.unwrap());

    // should check not None
    let option: Option<i32> = None;
    println!("{:?}", option.unwrap());
}
