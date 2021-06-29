fn main() {
    basic();
    println!("============");
    life_time();
    println!("============");
    slice_test();
    println!("============");
    move_semantics();
    println!("============");
    test_struct();
    println!("============");
    test_enum();
    println!("============");
    test_method();
    println!("============");
    test_option();
    println!("============");
    test_result();
    println!("============");
    test_trait();
}

fn basic() {
    println!("Hello, world!");
    let foo: (i32, i64) = (2, 3);
    println!("{}", foo.0);
    println!("{}", foo.1);

    let (foo_1, foo_2) = foo;
    println!("{}", foo_1);
    println!("{}", foo_2);

    let bar: [i32; 3] = [1, 2, 3];
    println!("{}", bar[0]);
    println!("{}", bar.len());

    println!("{}", 0x123);
    println!("{}", 0o123);
    println!("{}", 0b1010);

    let i: i32 = 32;
    println!("{}", i.pow(2));

    println!("{}", 1.4 + (2 as f32));

    let k: i32 = 300;
    let re = &k;
    println!("{}", re);
}

fn life_time() {
    let mut a = 123;
    let b = &a;
    println!("{}", b);
    a = 456;
    println!("{}", a);
}

fn slice_test() {
    let a: [i32; 3] = [1, 2, 3];
    let b: &[i32] = &a;
    println!("{:?}", b);

    let c: Vec<i32> = vec![4, 5, 6];
    let d: &[i32] = &c;
    println!("{:?}", d);

    println!("{:?}", &a[2..3]);
    println!("{:?}", &a[1..=2]);
    println!("{:?}", &c[2..]);
    println!("{:?}", &c[..3]);

    let s: String = String::from("hello");
    let t: &str = &s[0..2];
    println!("{}", t);
}

fn print_str(s: &String) {
    println!("{}", s);
}

fn move_semantics() {
    let s1 = String::from("Hello!");
    let s2 = s1;
    println!("{}", s2);
    // println!("{}", s1);

    let s = String::from("Hello");
    let ss: &String = &s;
    // let t = s;
    print_str(ss);
}

struct User {
    name: String,
    email: String,
    age: u8,
}

fn test_struct() {
    let user1 = User {
        name: String::from("Tanaka Taro"),
        email: String::from("tanaka@example.com"),
        age: 20,
    };
    println!("Welcom, {}!", user1.name);

    let mut user2 = User {
        name: String::from("Yamada Hanako"),
        email: String::from("yamada@example.com"),
        age: 32,
    };
    user2.email = String::from("yamada_hanako@example.com");
    println!("Welcom, {}!", user2.name);
    println!("Age, {}", user2.age);
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn move_to(step: i32, direction: &Direction) {
    match direction {
        Direction::Up => println!("move {} up", step),
        Direction::Down => println!("move {} down", step),
        Direction::Left => println!("move {} left", step),
        Direction::Right => println!("move {} right", step),
    };
}

fn test_enum() {
    let direction = Direction::Up;
    move_to(30, &direction);
    move_to(20, &Direction::Down);
    move_to(10, &Direction::Left);
    move_to(110, &Direction::Right);
}

struct Cat {
    name: String,
    age: u8,
}

impl Cat {
    fn generate_any_cat() -> Cat {
        return Cat {
            name: String::from("無名"),
            age: 8,
        };
    }

    fn mew(&self) {
        println!("{}({}): mew!", self.name, self.age);
    }

    fn eat(&self, food: &str) {
        println!("{}({}): eatng {}", self.name, self.age, food);
    }
}

fn test_method() {
    let cat = Cat {
        name: String::from("太郎"),
        age: 3,
    };
    cat.mew();
    cat.eat("cat foods");

    let cat2 = Cat::generate_any_cat();
    cat2.mew();
}

fn test_option() {
    let mut a: Option<u8> = None;
    println!("{:?}", a);
    a = Some(30);
    if let Some(n) = a {
        println!("{}", n);
    }
}

fn devide(a: isize, b: isize) -> Result<isize, String> {
    if b == 0 {
        return Err(String::from("can not deivde 0!!"));
    }
    Ok(a / b)
}

fn test_result() {
    let res1 = devide(100, 0);
    match res1 {
        Ok(n) => println!("{}", n),
        Err(m) => println!("{}", m),
    }
    let res2 = devide(100, 10);
    match res2 {
        Ok(n) => println!("{}", n),
        Err(m) => println!("{}", m),
    }
}

trait Animal {
    fn name(&self) -> &String;
    fn age(&self) -> u8;
    fn say(&self);
    fn eat(&self, food: String);
    fn human_age(&self) -> u8;
}

struct Dog {
    name: String,
    age: u8,
}

impl Animal for Dog {
    fn name(&self) -> &String {
        &self.name
    }

    fn age(&self) -> u8 {
        self.age
    }

    fn say(&self) {
        println!("{}({}): bow wow", self.name(), self.age());
    }

    fn eat(&self, food: String) {
        println!("{}({}): eating {}", self.name(), self.age(), food);
    }

    fn human_age(&self) -> u8 {
        self.age() * 7
    }
}

fn process(a: &dyn Animal){
    a.say();
    a.eat(String::from("Takoyaki"));
    println!("{} human age is {}", a.name(), a.human_age());
}

fn test_trait() {
    let dog = Dog {
        name: String::from("Jiro"),
        age: 4,
    };
    process(&dog);
}
