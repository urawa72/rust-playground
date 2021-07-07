fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rec1.area()
    );

    let rec2 = Rectangle {
        width: 20,
        height: 15,
    };
    let rec3 = Rectangle {
        width: 20,
        height: 60,
    };
    println!("Can rec1 hold rec2? {}", rec1.can_hold(&rec2));
    println!("Can rec1 hold rec3? {}", rec1.can_hold(&rec3));

    let rec4 = Rectangle::square(20);
    println!(
        "The area of the rectangle is {} square pixels.",
        rec4.area()
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 関連関数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
