fn main() {
    let mut s: String = String::new();
    // let result: std::io::Result<usize> = std::io::stdin().read_line(&mut s);
    // match result {
    //     Ok(_) => {
    //         println!("{}", s);
    //     }
    //     Err(err) => {
    //         println!("{}", err);
    //     }
    // }
    std::io::stdin().read_line(&mut s).ok();
    let v: Vec<u8> = s
        .trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect();

    println!("{}", v[1]);
}
