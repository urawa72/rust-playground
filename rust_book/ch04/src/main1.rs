fn main() {
    what_is_owenership();
}

fn what_is_owenership() {
    {
        // String literal
        let s = "hello";
        println!("{}", s);
        // String type
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
    }

    {
        // stack value is copied
        let x = 1;
        let y = x;
        println!("x is {}, y is {}", x, y);

        // heap value is not copied
        let s = String::from("hello");
        let t = s;
        // s is already moved
        // println!("s is {}", s);
        println!("t is {}", t);
    }

    {
        // depp copy
        let s = String::from("hello");
        let t = s.clone();
        println!("s is {}, t is {}", s, t);
    }

    {
        let s = String::from("hello");
        takes_owenership(s);
        // s is already moved
        // println!("s is {}", s);

        let x = 6;
        makes_copy(x);
        println!("x is {}", x);
    }

    {
        let s = gives_ownership();
        let t = String::from("hello");
        // t is moved here
        let u = takes_and_gives_back(t);
        println!("s is {}", s);
        // t is already moved thus can not use
        // println!("t is {}", t);
        println!("u is {}", u);
        // s, u is drop here
    }

    {
        let s = String::from("hello");
        // return same value to use same value is troublesome
        let (t, len) = calculate_length(s);
        println!("The length of '{}' is {}", t, len);
    }
}

fn takes_owenership(some_string: String) {
    println!("some_string is {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer is {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
