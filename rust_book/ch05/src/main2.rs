fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rec1)
    );
    println!("rec1 is {:?}", rec1);
    println!("rec1 is {:#?}", rec1);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}
