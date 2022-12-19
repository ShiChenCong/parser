// 关键字
pub enum Reserved {
    Null,
    True,
    False,
}

impl Reserved {
    pub fn name(&self) -> &str {
        match self {
            Reserved::Null => "null",
            Reserved::True => "true",
            Reserved::False => "false",
        }
    }

    pub fn into_string(self) -> String {
        self.name().to_string()
    }
}
