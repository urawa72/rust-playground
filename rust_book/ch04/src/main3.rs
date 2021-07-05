fn main() {
    {
        let mut s = String::from("hello world");
        let word = first_word(&s);
        s.clear();
        // not error but word is meaningless
        println!("word is {}", word);
    }

    {
        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];
        println!("s[0..5] is {}, s[6..11] is {}", hello, world);
    }

    {
        // can not borrow s as mutable after immutable borrow
        // let mut s = String::from("hello world");
        // let word = first_word_slice(&s);
        // s.clear();
        // println!("the first word is: {}", word);
    }

    {
        let my_s = String::from("hello world");
        let word = first_word_slice(&my_s[..]);
        println!("my_s first word is: {}", word);
        let my_s_literal = "hello world";
        let word = first_word_slice(&my_s_literal[..]);
        println!("my_s_literal slice first word is: {}", word);
        // string literal is string slice ref
        let word = first_word_slice(my_s_literal);
        println!("my_s_literal first word is: {}", word);
    }

    {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        println!("array slice is: {:?}", slice);
    }
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
