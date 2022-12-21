use crate::word::Reserved;

// 符号
pub enum TokenData {
    Reserved(Reserved),
    Equal,

    Identifier,
}
