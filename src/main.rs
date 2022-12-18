mod reader;

fn main() {
    let js_str = include_str!("./a.js");
    let mut a = reader::Reader::new(js_str.chars());
    // a.peek(5).unwrap();
    // a.next();
    a.log()
    // println!("{}", a.log())
}
