fn main() {
    let h = [1, 1, 0, 18];
    let s = hex(&h);
    println!("{:?}", s);
}

fn hex(bytes: &[u8]) -> String {
    println!("{:?} bytes", bytes.len());
    bytes
        .iter()
        .fold("".to_string(), |s, b| s + &format!("{:02X}", b))
}
