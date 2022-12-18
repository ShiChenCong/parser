use std::collections::VecDeque;

pub struct Reader<I> {
    chars: I,
    ahead: VecDeque<char>,
}

impl<I> Reader<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(chars: I) -> Reader<I> {
        Reader {
            chars,
            ahead: VecDeque::with_capacity(4),
        }
    }

    pub fn log(self) {
        println!("{:?}", self.ahead)
    }

    // 读取几个char
    pub fn peek(&mut self, n: usize) -> Option<char> {
        for _ in self.ahead.len()..(n + 1) {
            match self.chars.next() {
                // 读取到第几个就会往ahead里加几个
                Some(ch) => self.ahead.push_back(ch),
                None => return None,
            }
        }
        self.ahead.get(n).map(|&x| x)
    }
}

impl<I> Iterator for Reader<I>
where
    I: Iterator<Item = char>,
{
    type Item = char;
    fn next(&mut self) -> Option<char> {
        // peek之后的值都放到ahead里了
        let current_char = self.ahead.pop_front().or_else(|| self.chars.next());
        // 往后读取一位 也可能是换行
        if current_char == Some('\n') {
            println!("是换行");
        } else if current_char == Some('\r') {
            println!("是回车");
        } else {
            println!("{}", current_char.unwrap())
        }
        // 需要判断是不是换行
        current_char
    }
}
