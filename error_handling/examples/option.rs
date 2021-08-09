fn main() {
    let options = [Some(-1), None];

    for o in options.iter() {
        println!("Option: {:?}", o);

        let double = match o {
            Some(v) => v * 2,
            None => 0,
        };
        println!("match result: {}", double);

        if let Some(v) = o {
            println!("if let result: {}", v);
        }

        println!("");
    }
}
