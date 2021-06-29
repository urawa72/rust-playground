use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};

fn read_all() {
    let mut f = File::open("./sample.txt").expect("ファイルが見つかりませんでした");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("ファイルの入力に失敗しました");
    println!("{}", contents);
}

fn read_lines(){
    let f = File::open("./sample.txt").expect("ファイルが見つかりませんでした");
    for line in BufReader::new(f).lines() {
        println!("{}", line.unwrap());
    }
}

fn read_first_line() {
    let f = File::open("./sample.txt").expect("ファイルが見つかりませんでした");
    let mut buff = BufReader::new(f);
    let mut line = String::new();
    buff.read_line(&mut line).expect("ファイルの読み取りに失敗しました");
    println!("{}", line);
}

fn main() {
    read_all();
    read_lines();
    read_first_line();
}
