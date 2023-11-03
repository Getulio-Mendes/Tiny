use crate::lexical::Lexeme;
use crate::lexical::tokens::Token;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn lexical_analysis(f: File) -> Vec<Lexeme> {
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    let mut lexes = Vec::<Lexeme>::new();
    let mut line_number: u16 = 1;
    let mut next_c: char = ' ';

    loop {
        let lex = next_token(&mut reader, &mut line, &mut next_c, &mut line_number);
        //println!("({},{})",lex.token, Token::to_string(&lex.ttype).expect("Failed to parse"));

        if matches!(lex.ttype, Token::InvalidToken) {
            println!("Token inválido na linha {}", line_number);
        } else if matches!(lex.ttype, Token::UnexpectedEof) {
            println!("Fim de arquivo inesperado na linha {}", line_number);
        } else if matches!(lex.ttype, Token::EndOfFile) {
            lexes.push(lex);
            return lexes;
        }

        lexes.push(lex);
    }
}

fn next_token(
    reader: &mut BufReader<File>,
    line: &mut String,
    next_c: &mut char,
    line_number: &mut u16,
) -> Lexeme {
    let mut lex = Lexeme {
        token: "".to_string(),
        ttype: Token::EndOfFile,
        line_number: 0,
    };

    let mut state: i8 = 1;
    let mut c: char;

    while state != 7 && state != 8 {
        if line.is_empty() {
            reader.read_line(line).expect("Falha ao ler a linha");
        }

        if *next_c != ' ' {
            c = *next_c;
        } else {
            if line.len() > 1 {
                c = line.remove(0);
            } else {
                c = match line.pop() {
                    Some(x) => x,
                    None => {
                        if state == 4 {
                            lex.ttype = Token::UnexpectedEof;
                        } else {
                            lex.ttype = Token::EndOfFile;
                        }
                        lex.token = "".to_string();

                        // state 8
                        return lex;
                    }
                }
            }
        }

        *next_c = ' ';
        match state {
            1 => {
                if c == ' ' || c == '\t' || c == '\r' {
                    state = 1;
                } else if c == '#' {
                    state = 2;
                } else if c == '\n' {
                    *line_number += 1;
                    state = 1;
                } else if c == '=' || c == '<' || c == '>' {
                    lex.token.push_str(&c.to_string());
                    state = 3;
                } else if c == '!' {
                    lex.token.push_str(&c.to_string());
                    state = 4;
                } else if c == ';'
                    || c == '+'
                    || c == '-'
                    || c == '*'
                    || c == '/'
                    || c == '%'
                    || c == '^'
                {
                    lex.token.push_str(&c.to_string());
                    state = 7;
                }
                // var names
                else if c == '_' || c.is_alphabetic() {
                    lex.token.push_str(&c.to_string());
                    state = 5;
                } else if c.is_digit(10) {
                    lex.token.push_str(&c.to_string());
                    state = 6;
                } else {
                    lex.token.push_str(&c.to_string());
                    lex.ttype = Token::InvalidToken;
                    state = 8;
                }
            }

            2 => {
                if c == '\n' {
                    *line_number += 1;
                    state = 1;
                } else {
                    state = 2;
                }
            }

            3 => {
                if c == '=' {
                    lex.token.push_str(&c.to_string());
                    state = 7;
                } else {
                    *next_c = c;
                    state = 7;
                }
            }

            4 => {
                if c == '=' {
                    lex.token.push_str(&c.to_string());
                    state = 7;
                } else {
                    lex.ttype = Token::InvalidToken;
                    state = 8;
                }
            }

            5 => {
                if c == '_' || c.is_alphanumeric() {
                    lex.token.push_str(&c.to_string());
                    state = 5;
                } else {
                    *next_c = c;
                    state = 7;
                }
            }

            6 => {
                if c.is_digit(10) {
                    // Append the digit
                    lex.token.push_str(&c.to_string());
                    state = 6;
                } else {
                    // ignore the character
                    *next_c = c;
                    lex.ttype = Token::Number;
                    state = 8;
                }
            }

            _ => unreachable!(),
        }
    }

    if state == 7 {
        lex.ttype = Token::from_string(&lex.token);
    }

    lex.line_number = *line_number;
    return lex;
}
