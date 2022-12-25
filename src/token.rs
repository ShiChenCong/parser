use crate::word::Reserved;

// 集合
#[derive(Debug)]
pub enum TokenData {
    Reserved(Reserved),
    Token(Token),
    Identifier(String),
}

// 符号
#[derive(Clone, Copy, Debug)]
pub enum Token {
    Equal,
}
