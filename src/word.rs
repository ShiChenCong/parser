use std::collections::HashMap;

use crate::token::{Token, TokenData};

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
    token: HashMap<&'static str, Token>,
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
            token: wordmap!(Token, [("=", Equal)]),
        }
    }

    pub fn tokenize(&self, s: Word) -> Result<TokenData, ()> {
        Ok(match self.reserved.get(&s.text[..]) {
            Some(&word) => TokenData::Reserved(word),
            None => match self.token.get(&s.text[..]) {
                Some(&scc) => TokenData::Token(scc),
                None => TokenData::Identifier(s.text),
            },
        })
    }
}
