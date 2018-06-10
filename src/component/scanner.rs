// scanner.rs
// Copyright 2018 AnonymousDapper

use logger;

use component::token::{Token, TokenType, LiteralType};

pub struct Scanner {
  source: String,
  iter: Vec<char>,
  index: u32,
  line: u32,
  column: u32,
  tokens: Vec<Token>
}

impl Scanner {
  pub fn new(src: String) -> Scanner {
    Scanner {
      source: src.clone(),
      iter: src.chars().collect::<Vec<char>>(),
      index: 0,
      line: 1,
      column: 1,
      tokens: Vec::<Token>::new()
    }
  }

  // temporary function for logging syntax errors
  fn invalid_char(&self, c: char) {
    logger::error(format!("Unexpected '{}'", c), self.line, self.column);
  }

  // add token without literal value
  fn add_token(&mut self, token_type: TokenType) {
    //println!("Adding {:?}", token_type);
    self.tokens.push(Token::new(token_type, LiteralType::Other, self.line, "".to_owned()));
  }

  // add token with a literal value
  fn add_token_literal(&mut self, literal: LiteralType, lexeme: String) {
    self.tokens.push(Token::new(TokenType::Literal, literal, self.line, lexeme));
  }

  // check how far we are in source
  fn at_end(&self) -> bool {
    //println!("index: {}, at end: {}", self.index, self.index > (self.iter.len() - 1) as u32);
    self.index > (self.iter.len() - 1) as u32
  }

  // increment our index and return the character
  fn next(&mut self) -> char {
    self.index += 1;
    //println!("next index: {}", self.index);
    if !self.at_end() {
      self.iter[self.index as usize]
    } else {
      '\0'
    }
  }

  // take a peek at the next character
  fn peek(&self) -> char {
    let peek_idx = (self.index + 1);

    if !self.at_end() {
      self.iter[peek_idx as usize]
    } else {
      '\0'
    }
  }

  // go back one character
  fn rewind(&mut self) {
    self.index -= 1;
  }

  // get current character
  fn current(&self) -> char {
    self.iter[self.index as usize]
  }

  // parse a binary number from source
  fn consume_binary(&mut self) {
    use std::i32;

    let mut buf = String::new();
    let mut c = self.current();
    while c.is_digit(2) {
      buf.push(c);
      c = self.next();
    };

    if !buf.is_empty() {
      if let Ok(result) = i32::from_str_radix(&buf, 2) {
        self.add_token_literal(LiteralType::Int(result), buf);
      } else {
        logger::error("bad number", self.line, self.column);
      };
    } else {
      logger::error("no number", self.line, self.column);
    }
  }

  // parse a hexadecimal number from source
  fn consume_hex(&mut self) {
    use std::i32;

    let mut buf = String::new();
    let mut c = self.current();
    while c.is_digit(16) {
      buf.push(c);
      c = self.next();
    };

    if !buf.is_empty() {
      if let Ok(result) = i32::from_str_radix(&buf, 16) {
        self.add_token_literal(LiteralType::Int(result), buf);
      } else {
        logger::error("bad number", self.line, self.column);
      };
    } else {
      logger::error("no number", self.line, self.column);
    }
  }

  // parse an octal number from source
  fn consume_octal(&mut self) {
    use std::i32;

    let mut buf = String::new();
    let mut c = self.current();
    while c.is_digit(8) {
      buf.push(c);
      c = self.next();
    };

    if !buf.is_empty() {
      if let Ok(result) = i32::from_str_radix(&buf, 8) {
        self.add_token_literal(LiteralType::Int(result), buf);
      } else {
        logger::error("bad number", self.line, self.column);
      };
    } else {
      logger::error("no number", self.line, self.column);
    }
  }

  // scan a token
  fn scan_token(&mut self) {
    use self::TokenType::*;

    let c = self.current();
    //println!("c: {}", c);

    match c {
      ' ' | '\t' => { println!("space"); self.next(); },
      '\n' => self.line += 1,
      '(' => { self.next(); self.add_token(LeftParen) },
      ')' => { self.next(); self.add_token(RightParen) },
      '[' => { self.next(); self.add_token(LeftBracket) },
      ']' => { self.next(); self.add_token(RightBracket) },
      '{' => { self.next(); self.add_token(LeftBrace) },
      '}' => { self.next(); self.add_token(RightBrace) },
      '+' => {
        let token_type = match self.next() {
          '+' => Increment,
          '=' => InAdd,
          _ => { print!(" next: '{}' ", self.current()); Add }
        };
        self.add_token(token_type);
      },
      '-' => {
        let token_type = match self.next() {
          '-' => Decrement,
          '=' => InSubtract,
          _   => Subtract
        };
        self.add_token(token_type);
      },
      '/' => {
        let token_type = match self.next() {
          '=' => InDivide,
          _   => Divide
        };
        self.add_token(token_type);
      },
      '*' => {
        let token_type = match self.next() {
          '*' => Pow,
          '=' => InMultiply,
          _   => Multiply
        };
        self.add_token(token_type);
      },
      '%' => {
        let token_type = match self.next() {
          '=' => InModulo,
          _   => Modulo
        };
        self.add_token(token_type);
      },
      '>' => {
        let token_type = match self.next() {
          '>' => match self.next() {
            '=' => InBitRShift,
            _   => BitRightShift
          },
          '=' => GtOrEqualTo,
          _   => GreaterThan
        };
        self.add_token(token_type);
      },
      '<' => {
        let token_type = match self.next() {
          '<' => match self.next() {
            '=' => InBitLShift,
            _   => BitLeftShift
          },
        '=' => LtOrEqualTo,
        _   => LessThan
        };
        self.add_token(token_type);
      },
      '&' => {
        let token_type = match self.next() {
          '=' => InBitAnd,
          _   => BitAnd
        };
        self.add_token(token_type);
      },
      '|' => {
        let token_type = match self.next() {
          '=' => InBitOr,
          _   => BitOr
        };
        self.add_token(token_type);
      },
      '^' => {
        let token_type = match self.next() {
          '=' => InBitXOr,
          _   => BitXOr
        };
        self.add_token(token_type);
      },
      '!' => {
        let token_type = match self.next() {
          '=' => InBitNot,
          _   => Exclaim
        };
        self.add_token(token_type);
      },
      '=' => {
        let token_type = match self.next() {
          '=' => Equals,
          '/' => {
            match self.next() {
              '=' => NotEquals,
              other => { self.invalid_char(other); Error}
            }
          },
          _ => Assign
        };
        self.add_token(token_type);
      },
      ',' => self.add_token(Comma),
      '0' => {
        match self.next() {
          'b' | 'B' => { self.next(); self.consume_binary() },
          'o' | 'O' => { self.next(); self.consume_octal() },
          'x' | 'X' => { self.next(); self.consume_hex() },
          _ => { self.rewind(); }
        };
      }

      other => {
        /*if c.is_digit(10) || c == '.' {
          self.rewind();
          println!("num");
          self.consume_number();

        } else {*/
          self.invalid_char(other);
          self.add_token(Error);
          self.next();
        //}
      }
    };

    self.column += 1;
  }

  // run scan on whole source
  pub fn scan(mut self) -> Vec<Token> {
    while !self.at_end() {
      //println!("index: {}, current: '{}' ", self.index, self.current());
      self.scan_token();
    }

    self.add_token(TokenType::EoF);
    self.tokens
  }
}
