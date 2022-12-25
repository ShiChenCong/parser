use std::process;

use crate::token::TokenData;
use crate::word;
use crate::{reader::Reader, word::Word};

pub struct Lexer<I> {
    pub reader: Reader<I>,
    wordmap: word::Map,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(chars: I) -> Lexer<I> {
        Lexer {
            reader: Reader::new(chars),
            wordmap: word::Map::new(),
        }
    }

    //  取一个char
    fn peek(&mut self) -> Option<char> {
        self.reader.peek(0)
    }

    // 取两个char
    fn peek2(&mut self) -> (Option<char>, Option<char>) {
        (self.reader.peek(0), self.reader.peek(1))
    }

    fn read(&mut self) -> char {
        self.reader.next().unwrap()
    }

    fn read_word(&mut self) -> Result<TokenData, ()> {
        let s = self.read_word_parts().unwrap();
        self.wordmap.tokenize(s)
    }

    // 读取完整的一个word
    fn read_word_parts(&mut self) -> Result<Word, ()> {
        // 读取完整的单词直到空
        // self.read_until_with(pred, read)
        let mut s = Word::new();
        self.read_until_with(
            // 空格的时候就不读了
            |ch| ch.is_whitespace(),
            &mut |this| match this.read() {
                '\\' => Ok(()),
                ch => {
                    s.text.push(ch);
                    Ok(())
                }
            },
        )?;
        Ok(s)
    }

    fn read_until_with<F, G>(&mut self, pred: F, read: &mut G) -> Result<(), ()>
    where
        F: Fn(char) -> bool,
        G: FnMut(&mut Self) -> Result<(), ()>,
    {
        loop {
            match self.peek() {
                Some(ch) if pred(ch) => {
                    self.reader.next();
                    return Ok(());
                }
                Some(_) => {
                    read(self)?;
                }
                None => return Ok(()),
            }
        }
    }

    pub fn start(&mut self) {
        loop {
            let res = self.read_token().unwrap();
            println!("{:?}", res);
        }
    }

    pub fn read_token(&mut self) -> Result<TokenData, ()> {
        // 为什么需要读取两个， 因为符号是两个 // <! 等
        let pair = self.peek2();
        // 先判断是不是数字 再判断是不是
        let result = match pair {
            // 如果是数字
            (Some(_ch), _) => self.read_word(),
            // 如果是关键字或者identifier
            (None, _) => {
                println!("end");
                process::exit(1);
            }
        };
        result
    }
}
