use std::{env, fs::File, io::BufReader};

use wordcount::count;

fn main() {
    println!("Hello, world!");
    let filename = env::args().nth(1).expect("1 argument FILENAME required");
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(&file);

    let freqs = count(reader, Default::default());
    println!("{:?}", freqs);
}
