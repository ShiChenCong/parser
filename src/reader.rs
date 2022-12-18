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
        let current_char = self.ahead.pop_front().or_else(|| self.chars.next());
        // 往后读取一位 也可能是换行
        current_char
    }
}
