fn main() {
    {
        let s = String::from("hello");
        println!("The length of '{}' is {}", s, calc_length(&s));
    }

    {
        let mut s = String::from("hello");
        let r1 = &mut s;
        // let r2 = &mut s;
        println!("{}", r1);

        multiple_ref();
    }

    {
        let mut s = String::from("hello");
        let r1 = &s;
        println!("r1 is {}", r1);
        let r2 = &s;
        println!("r2 is {}", r2);
        let r3 = &mut s;
        println!("r3 is {}", r3);
    }

    let ref_nothing = dangle();
    println!("dangle sample: {}", ref_nothing);
}

fn calc_length(s: &String) -> usize {
    s.len()
}

fn multiple_ref() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1 is {}", r1);
    } // drop r1 here
    let r2 = &mut s;
    println!("r2 is {}", r2);
}

// fn dangle() -> &String {
fn dangle() -> String {
    let s = String::from("hello");
    s // ownership move
      // &s
}
