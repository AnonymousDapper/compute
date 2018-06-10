// reporter
// Copyright 2018 AnonymousDapper

mod position {
  use std::fmt;

  #[derive(PartialOrd, PartialEq, Ord, Eq)]
  pub struct Position {
    pub line: u32,
    pub column: u32
  }

  impl Position {
    pub fn new(line: u32, column: u32) -> Position {
      Position {
        line,
        column
      }
    }

    pub fn span_to(self, to: Position) -> Span {
      Span::new(self, to)
    }

    pub fn forward(mut self, amount: u32) -> Position {
      self.column += amount;
      self
    }

    pub fn backward(mut self, amount: u32) -> Position {
      assert!(self.column >= amount, "too far back");
      self.column -= amount;
      self
    }
  }

  impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "line {} col {}", self.line, self.column)
    }
  }

  pub struct Span {
    pub start: Position,
    pub end: Position
  }

  impl Span {
    pub fn new(start: Position, end: Position) -> Span {
      assert!(start <= end);
      Span {
        start,
        end
      }
    }

    pub fn merge(self, other: Span) -> Span {
      use std::cmp;

      let start = cmp::min(self.start, other.start);
      let end = cmp::max(self.end, other.end);
      Span::new(start, end)
    }
  }
}

enum Severity {
  Error,
  Warning,
  Message,
  Debug
}

fn write<T>(msg: T, severity: Severity, line: u32, column: u32) where T: Into<String>{
  let message = msg.into();

  match severity {
    Severity::Error => println!("\x1b[91;1m! Error [line {} col {}] !\n==>\x1b[0m {}\n", line, column, message),
    Severity::Warning => println!("\x1b[93;1m* Warning [line {} col {}] *\n==>\x1b[0m {}\n", line, column, message),
    Severity::Message => println!("\x1b[36m< Info [line {} col {}] >\n==>\x1b[0m {}\n", line, column, message),
    Severity::Debug => println!("\x1b[90m( Debug [line {} col {}] )\n==>\x1b[0ms {}\n", line, column, message)
  };
}

pub fn error<T>(msg: T, line: u32, column: u32) where T: Into<String> {
  write(msg, Severity::Error, line, column);
}

pub fn warn<T>(msg: T, line: u32, column: u32) where T: Into<String> {
  write(msg, Severity::Warning, line, column);
}

pub fn log<T>(msg: T, line: u32, column: u32) where T: Into<String> {
  write(msg, Severity::Message, line, column);
}

pub fn debug<T>(msg: T, line: u32, column: u32) where T: Into<String> {
  write(msg, Severity::Debug, line, column);
}