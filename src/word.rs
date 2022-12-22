use std::collections::HashMap;

use crate::token::TokenData;

pub struct Word {
    had_escape: bool,
    pub text: String,
}

impl Word {
    pub fn new() -> Self {
        Word {
            had_escape: false,
            text: String::new(),
        }
    }
}
// 关键字
#[derive(Clone, Copy, Debug)]
pub enum Reserved {
    Null,
    True,
    False,
    Const,
}

impl Reserved {
    pub fn name(&self) -> &str {
        match self {
            Reserved::Null => "null",
            Reserved::True => "true",
            Reserved::False => "false",
            Reserved::Const => "const",
        }
    }

    pub fn into_string(self) -> String {
        self.name().to_string()
    }
}

macro_rules! wordmap{
    ($ns:ident, [$(($key:expr, $val:ident)), *]) => {
        {
            let mut temp_map = HashMap::new();
            $(
                temp_map.insert($key, $ns::$val);
            )*
            temp_map
        }
    };
}

pub struct Map {
    reserved: HashMap<&'static str, Reserved>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            reserved: wordmap!(
                Reserved,
                [
                    ("null", Null),
                    ("true", True),
                    ("false", False),
                    ("const", Const)
                ]
            ),
        }
    }

    pub fn tokenize(&self, s: Word) -> Result<TokenData, ()> {
        Ok(match self.reserved.get(&s.text[..]) {
            // 关键字 或者 符号
            Some(&word) => TokenData::Reserved(word),
            None => {
                // 那就是变量名
                TokenData::Identifier(s.text)
            }
        })
    }
}
