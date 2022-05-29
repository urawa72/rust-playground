use json_parser::Lexer;

fn main() {
    let arr = "[true, {\"キー\": null}]";
    let result = Lexer::new(arr).tokenize().unwrap();
    println!("{:?}", result);
}
