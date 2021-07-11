use std::{
    fs::File,
    io::{Error, ErrorKind, Read},
};

fn main() {
    // sample01();
    // sample02();
    // sample03();
    // let r = read_username_from_file01();
    // match r {
    //     Ok(_) => println!("OK!!!!!!!!!!"),
    //     Err(_) => panic!("Error!!!!!!!!!!!!!"),
    // }
    let r = read_username_from_file02();
    println!("{:?}", r);
}

fn sample01() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };
    println!("{:?}", f);
}

fn sample02() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
        },
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };
    println!("{:?}", f);
}

fn sample03() {
    // let f = File::open("hello03.txt").unwrap();
    // println!("{:?}", f);
    let f = File::open("hello03.txt").expect("Faild to opne hello03.txt");
    println!("{:?}", f);
}

fn read_username_from_file01() -> Result<String, Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file02() -> Result<String, Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
