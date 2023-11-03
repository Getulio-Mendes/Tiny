pub mod lexical_analizer;
pub mod tokens;
use crate::lexical::tokens::Token;

pub struct Lexeme {
  pub ttype: Token,
  pub token: String,
  pub line_number: u16,
}