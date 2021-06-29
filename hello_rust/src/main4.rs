use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};

fn write_all() {
    let mut file = File::create("./output.txt").expect("ファイル出力に失敗");
    writeln!(file, "hello").expect("書き出しエラー");
}

fn write_append() {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./output.txt")
        .expect("ファイル作成エラー");
    writeln!(file, "world").expect("書き出しエラー");
}

fn write_buffering() {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./output.txt")
        .expect("ファイル作成エラー");
    let mut buf_writer = BufWriter::new(file);
    for _ in 0..1_000_000 {
        writeln!(buf_writer, "!!!!!!!!!").expect("書き出しエラー");
    }
}

fn main() {
    write_all();
    write_append();
    write_buffering();
}
