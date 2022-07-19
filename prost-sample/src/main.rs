use prost_sample::{create_large_shirt, deserialize_shirt, serialize_shirt};

fn main() {
    let s = create_large_shirt("red".to_string());
    println!("{:?}", s);

    let v = serialize_shirt(&s);
    println!("{:?}", v);

    let b = deserialize_shirt(&v).unwrap();
    println!("{:?}", b);
}
