use lexer::Lexer;

mod lexer;
mod reader;
mod token;
mod word;

fn main() {
    let js_str = include_str!("./a.js");
    // let mut a = reader::Reader::new(js_str.chars());
    let mut a = Lexer::new(js_str.chars());
    a.start();
}
