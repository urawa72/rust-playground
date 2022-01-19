use regex::Regex;

fn main() {
    let val = "1234567890123456".to_string();
    match check(&val) {
        Ok(code) => println!("{:?}", code),
        Err(e) => println!("{:?}", e),
    }

}

fn check(val: &String) -> Result<Code, CustomError> {
    let code = Code::new(val)?;
    Ok(code)
}

#[derive(Debug)]
enum CustomError {
    RegexError
}

#[derive(Debug)]
struct Code(String);

impl Code {
    pub fn new(val: &String) -> Result<Self, CustomError> {
        let re = Regex::new(r"\d{16}").unwrap();
        if re.is_match(val.as_str()) {
            Ok(Self(val.to_owned()))
        } else {
            Err(CustomError::RegexError)
        }
    }
}
