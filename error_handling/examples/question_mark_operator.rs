fn main() {
    let odd = |n| -> Result<i32, String> {
        if n % 2 == 1 {
            Ok(n)
        } else {
            Err(format!("{} is not odd", n))
        }
    };

    let double = |n| -> Result<i32, String> {
        let n = odd(n)?;
        // panic if got Err by unwrap()
        // let n = odd(n).unwrap();
        Ok(n * 2)
    };

    for n in 0..4 {
        println!("number: {}", n);
        println!("double result: {:?}\n", double(n));
    }

    let double = |n: Option<i32>| -> Option<i32> {
        println!("Option: {:?}", n);
        let m = n?;
        // panic if got None by unwrap()
        // let m = n.unwrap();
        Some(m * 2)
    };

    println!("{:?}\n", double(None));
    println!("{:?}\n", double(Some(100)));
}
