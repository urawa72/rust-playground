use std::fmt::Display;

fn main() {
    lifetime_01();
    lifetime_02();
    // lifetime_03();
    all();
}

fn lifetime_01() {
    let s1 = "abcd".to_string();
    let s2 = "xyz";
    let r = longest(s1.as_str(), s2);
    println!("The longest string is {}", r);
}

fn lifetime_02() {
    let s1 = "long string is long".to_string();
    {
        let s2 = "xyz";
        let r = longest(s1.as_str(), s2);
        println!("The longest string is {}", r);
    }
}

// fn lifetime_03() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
// }

fn all() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest_with_an_announcement(string1.as_str(), string2.as_str(), "hello");
        println!("The longest string is {}", result);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
