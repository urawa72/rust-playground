fn main() {
    let results = [Ok(100), Err("Error")];

    println!("{:?}", results[0].map(|n| n * 2));
    println!("{:?}", results[1].map(|n| n * 2));

    let f = |n| Ok(n * 2);
    println!("{:?}", results[0].and_then(f));
    println!("{:?}", results[1].and_then(f));

    let f = |n| Ok(format!("number is {}", n));
    println!("{:?}", results[0].and_then(f));

    println!("{:?}", results[0].ok());
    println!("{:?}", results[0].err());
    println!("{:?}", results[1].ok());
    println!("{:?}", results[1].err());
}
