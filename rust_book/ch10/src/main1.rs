mod lib;
use crate::lib::{notify, Summary};
use lib::{Pair, Tweet};

fn main() {
    check_largest();
    struct_generics();
    method_generics();
    trait_sample();
    let r = return_summarize();
    notify(&r);
    trait_boundary();
}

fn check_largest() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The larget{}", result);
    let number_list = vec![34, 50, 25, 100, 65, 300, 45];
    let result = largest_copy(&number_list);
    println!("The largest number is {}", result);
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn struct_generics() {
    {
        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.x());
    }

    {
        let p = Point { x: 3.5, y: 4.9 };
        println!("(0, 0) distance is {}", p.distance_from_origin());
    }
}

fn method_generics() {
    struct Point2<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point2<T, U> {
        fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
            Point2 {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point2 { x: 5, y: 2.5 };
    let p2 = Point2 { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn trait_sample() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet)
}

fn return_summarize() -> impl Summary {
    Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    }
}

fn trait_boundary() {
    let p = Pair::new(4, 10);
    p.cmp_display();
}
