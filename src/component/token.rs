// token.rs
// Copyright 2018 AnonymousDapper

use std::fmt;

#[derive(Debug)]
pub enum TokenType {
  // one character
  LeftParen,      // (
  RightParen,     // )
  LeftBracket,    // [
  RightBracket,   // ]
  LeftBrace,      // {
  RightBrace,     // }
  Add,            // +
  Subtract,       // -
  Divide,         // /
  Multiply,       // *
  Modulo,         // %
  GreaterThan,    // >
  LessThan,       // <
  BitAnd,         // &
  BitOr,          // |
  BitXOr,         // ^
  Exclaim,        // !
  Assign,         // =
  Comma,          // ,

  // two characters
  BitLeftShift,  // <<
  BitRightShift, // >>
  Pow,           // **
  GtOrEqualTo,   // >=
  LtOrEqualTo,   // <=
  Equals,        // ==
  Increment,     // ++
  Decrement,     // --
  InAdd,         // +=
  InSubtract,    // -=
  InDivide,      // /=
  InMultiply,    // *=
  InModulo,      // %=
  InBitAnd,      // &=
  InBitOr,       // |=
  InBitXOr,      // ^=
  InBitNot,     // !=

  // three characters
  NotEquals,     // =/=
  InBitLShift,   // <<=
  InBitRShift,   // >>=

  // other
  Literal,
  Error,
  EoF
}

impl fmt::Display for TokenType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub enum LiteralType {
  Identifier(String),
  Float(f32),
  Int(i32),
  SciNotFloat(f32),
  SciNotInt(i32),
  Other
}

impl fmt::Display for LiteralType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      LiteralType::Other => write!(f, ""),
      LiteralType::Identifier(ref v) => write!(f, "Ident({})", v),
      LiteralType::Float(ref v) => write!(f, "Float({})", v),
      LiteralType::Int(ref v) => write!(f, "Int({})", v),
      LiteralType::SciNotFloat(ref v) => write!(f, "SciNotInt({})", v),
      LiteralType::SciNotInt(ref v) => write!(f, "SciNotFloat({})", v)
    }
  }
}

pub struct Token {
  pub token_type: TokenType,
  pub literal: LiteralType,
  pub line: u32,
  pub lexeme: String
}

impl Token {
  pub fn new(token_type: TokenType, literal: LiteralType, line: u32, lexeme: String) -> Token {
    Token {
      token_type,
      literal,
      line,
      lexeme
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
  }
}
