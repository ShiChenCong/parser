mod reader;

fn main() {
    let js_str = include_str!("./a.js");
    let mut a = reader::Reader::new(js_str.chars());
    // a.peek(10);
    a.next();
    a.next();
    a.next();
    a.next();
    a.next();
    // let n1 = a.next().unwrap();
    // let n2 = a.next().unwrap();
    // let n3 = a.next().unwrap();
    // a.peek(5).unwrap();
    // a.next();
    // a.log();
    // println!("{},{}", n1, n2)
}
