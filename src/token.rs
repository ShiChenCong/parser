use crate::word::Reserved;

// 符号
#[derive(Debug)]
pub enum TokenData {
    Reserved(Reserved),
    Equal,
    WhiteSpace,

    Identifier(String),
}
