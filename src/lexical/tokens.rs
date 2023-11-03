pub enum Token {
    // Specials
    UnexpectedEof = -2,
    InvalidToken = -1,
    EndOfFile = 0,

    // Symbols
    Semicolon, // ;
    Assign,    // =

    // Logic operators
    Equal,        // ==
    NotEqual,     // !=
    Lower,        // <
    LowerEqual,   // <=
    Greater,      // >
    GreaterEqual, // >=

    // Arithmetic operators
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %
    Pow, // ^

    // Keywords
    Program, // program
    While,   // while
    Do,      // do
    Done,    // done
    If,      // if
    Then,    // then
    Else,    // else
    Output,  // output
    True,    // true
    False,   // false
    Read,    // read
    Not,     // not

    // Others
    Number, // number
    Var,    // variable
}

impl Token {
    pub fn to_string(&self) -> Result<&str, &str> {
        match self {
            Token::UnexpectedEof => Ok("UNEXPECTED_EOF"),
            Token::InvalidToken => Ok("INVALID_TOKEN"),
            Token::EndOfFile => Ok("EOF"),
            Token::Semicolon => Ok("SEMICOLON"),
            Token::Assign => Ok("ASSIGN"),
            Token::Equal => Ok("EQUAL"),
            Token::NotEqual => Ok("NOT_EQUAL"),
            Token::Lower => Ok("LOWER"),
            Token::LowerEqual => Ok("LOWER_EQUAL"),
            Token::Greater => Ok("GREATER"),
            Token::GreaterEqual => Ok("GREATER_EQUAL"),
            Token::Add => Ok("ADD"),
            Token::Sub => Ok("SUB"),
            Token::Mul => Ok("MUL"),
            Token::Div => Ok("DIV"),
            Token::Mod => Ok("MOD"),
            Token::Pow => Ok("POW"),
            Token::Program => Ok("PROGRAM"),
            Token::While => Ok("WHILE"),
            Token::Do => Ok("DO"),
            Token::Done => Ok("DONE"),
            Token::If => Ok("IF"),
            Token::Then => Ok("THEN"),
            Token::Else => Ok("ELSE"),
            Token::Output => Ok("OUTPUT"),
            Token::True => Ok("TRUE"),
            Token::False => Ok("FALSE"),
            Token::Read => Ok("READ"),
            Token::Not => Ok("NOT"),
            Token::Number => Ok("NUMBER"),
            Token::Var => Ok("VAR"),
        }
    }

    pub fn from_string(s: &str) -> Token {
        match s {
            ";" => Token::Semicolon,
            "=" => Token::Assign,
            "==" => Token::Equal,
            "!=" => Token::NotEqual,
            "<" => Token::Lower,
            "<=" => Token::LowerEqual,
            ">" => Token::Greater,
            ">=" => Token::GreaterEqual,
            "+" => Token::Add,
            "-" => Token::Sub,
            "*" => Token::Mul,
            "/=" => Token::Div,
            "%" => Token::Mod,
            "^" => Token::Pow,
            "program" => Token::Program,
            "while" => Token::While,
            "do" => Token::Do,
            "done" => Token::Done,
            "if" => Token::If,
            "then" => Token::Then,
            "else" => Token::Else,
            "output" => Token::Output,
            "true" => Token::True,
            "false" => Token::False,
            "read" => Token::Read,
            "not" => Token::Not,
            _ => Token::Var,
        }
    }
}
