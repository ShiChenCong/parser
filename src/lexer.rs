use crate::reader::Reader;

pub struct Lexer<I> {
    reader: Reader<I>,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    fn peek2(&mut self) -> (Option<char>, Option<char>) {
        (self.reader.peek(0), self.reader.peek(1))
    }

    fn read_word(&mut self) {}

    fn read_token(&mut self) {
        let pair = self.peek2();
        // 先判断是不是数字 再判断是不是
        let mut result = match pair {
            // 如果是数字
            (Some(ch), _) => self.read_word(),
            // 如果是关键字或者identifier
            (Some(ch), _) => self.read_word(),
            (None, _) => {}
        };
        result
    }
}
