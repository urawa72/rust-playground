fn main() {
    let results = [Ok(100), Err("oops!")];

    for r in results.iter() {
        println!("Result: {:?}", r);

        let double = match r {
            Ok(v) => v * 2,
            Err(_) => 0,
        };
        println!("match result: {}", double);

        if let Ok(v) = r {
            println!("if let result: {}", v);
        }

        println!("");
    }
}
