use crate::ast::identifier::Identifier;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("token error: {0}")]
pub struct TokenError(&'static str);
pub type TokenResult<T> = Result<T, TokenError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimiter {
  OpenBracket,
  CloseBracket,
  OpenCurly,
  CloseCurly,
  DoubleQuote,
  Comma,
}

impl Delimiter {
  fn str(&self) -> &'static str {
    use Delimiter::*;
    match self {
      OpenBracket => "(",
      CloseBracket => ")",
      OpenCurly => "{",
      CloseCurly => "}",
      DoubleQuote => "\"",
      Comma => ",",
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
  Assign,
  Equals,
  Divide,
  Multiply,
  Add,
  Subtract,
  And,
  Or,
}

impl Operator {
  fn str(&self) -> &'static str {
    use Operator::*;
    match self {
      Assign => "=",
      Equals => "==",
      Divide => "*",
      Multiply => "/",
      Add => "-",
      Subtract => "+",
      And => "&&",
      Or => "||",
    }
  }

  pub fn operators() -> &'static [Operator] {
    use Operator::*;
    &[Assign, Equals, Divide, Multiply, Add, Subtract, And, Or]
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
  If,
  Else,
  Fn,
  True,
  False,
}

impl Keyword {
  fn str(&self) -> &'static str {
    use Keyword::*;
    match self {
      If => "if",
      Else => "else",
      Fn => "fn",
      True => "true",
      False => "false",
    }
  }
}

pub struct TokenStream<'a> {
  // position of the next character, starts at the final index. `None` if at the end of the string.
  next_position: Option<usize>,
  string: &'a str,
}

impl<'a> TokenStream<'a> {
  pub fn new(string: &'a str) -> Self {
    TokenStream {
      next_position: if string.is_empty() {
        None
      } else {
        Some(string.len() - 1)
      },
      string,
    }
  }

  fn peek_next_n(&mut self, n: usize) -> Option<&'a str> {
    self
      .next_position
      .and_then(|next_pos| self.string.get(next_pos - n..next_pos)) // TODO: might fail with negative numbers
  }

  fn consume_next_n(&mut self, n: usize) -> Option<&'a str> {
    if let Some(next_pos) = self.next_position {
      let char = self
        .string
        .get(next_pos - n..next_pos)
        .expect("next_position should always be valid");
      if next_pos >= n {
        self.next_position = Some(next_pos - n);
      } else {
        self.next_position = None;
      }
      Some(char)
    } else {
      None
    }
  }

  fn peek_next_char(&mut self) -> Option<char> {
    self.peek_next_n(1).and_then(|str| str.chars().nth(0))
  }

  fn consume_next_char(&mut self) -> Option<char> {
    self.consume_next_n(1).and_then(|str| str.chars().nth(0))
  }

  // Skip any comments or whitespace
  pub fn skip_noop(&mut self) {
    if let Some(next_char) = self.peek_next_char() {
      if next_char.is_whitespace() {
        // consume the whitespace, then repeat.
        self.consume_next_char();
        return self.skip_noop();
      } else if next_char == '\\' && self.peek_next_n(2) == Some("\\\\") {
        // start of a comment, read until the end of the line
        loop {
          match self.consume_next_char() {
            Some('\n') | None => break,
            _ => continue,
          }
        }
      }
    }
  }

  pub fn is_empty(&self) -> bool {
    self.next_position.is_none()
  }

  pub fn try_identifier_opt(&mut self) -> TokenResult<Option<Identifier<'a>>> {
    self.skip_noop();
    for n in 1.. {
      let str = match self.peek_next_n(n) {
        Some(str) => str,
        None => return Ok(None),
      };

      let char = str.chars().nth(0).unwrap();
      if n == 1 {
        if !Identifier::is_valid_first_char(char) {
          return Err(TokenError(
            "invalid first character of identifier, must only be alphabetic or _",
          ));
        }
      } else if !Identifier::is_valid_char(char) {
        // end of identifier
        return Ok(Some(Identifier(self.consume_next_n(n - 1).unwrap())));
      }
    }
    unreachable!()
  }

  pub fn try_number_opt(&mut self) -> TokenResult<Option<f64>> {
    self.skip_noop();
    let mut had_decimal = false; // whether a decimal has already been seen
    for n in 1.. {
      let str = match self.peek_next_n(n) {
        Some(str) => str,
        None => return Ok(None),
      };

      let char = str.chars().nth(0).unwrap();
      if n == 1 && !char.is_numeric() {
        return Ok(None);
      } else if char.is_numeric() {
        continue;
      } else if char == '.' {
        if n == 1 {
          return Err(TokenError("number cannot end in decimal"));
        } else if had_decimal {
          return Err(TokenError("invalid number, cannot have multiple decimals"));
        } else {
          had_decimal = true;
        }
      } else {
        // end of number
        let number_str = self.consume_next_n(n - 1).unwrap();
        return Ok(Some(
          number_str
            .parse()
            .expect("number parsing restrictions should result in valid float"),
        ));
      }
    }
    unreachable!()
  }

  pub fn try_string_opt(&mut self) -> TokenResult<Option<&'a str>> {
    self.skip_noop();

    if self.try_chars(Delimiter::DoubleQuote.str()).is_err() {
      return Ok(None);
    }
    for n in 1.. {
      let str = self
        .peek_next_n(n)
        .ok_or(TokenError("expected string, found nothing"))?;

      // TODO: unsure what an unfinished string will do here
      let char = str.chars().nth(0).unwrap();
      if char == Delimiter::DoubleQuote.str().chars().next().unwrap() {
        let inner_str = self.consume_next_n(n - 1).unwrap();
        return Ok(Some(inner_str));
      }
    }
    unreachable!()
  }

  fn try_chars(&mut self, str: &str) -> TokenResult<&'_ str> {
    self.skip_noop();

    if self.peek_next_n(str.len()) == Some(str) {
      self.consume_next_n(str.len());
      Ok(str)
    } else {
      Err(TokenError("expected chars"))
    }
  }

  pub fn try_operator(&mut self, operator: Operator) -> TokenResult<Operator> {
    self.try_chars(operator.str()).map(|_| operator)
  }

  pub fn try_delimiter(&mut self, delimiter: Delimiter) -> TokenResult<Delimiter> {
    self.try_chars(delimiter.str()).map(|_| delimiter)
  }

  pub fn try_keyword(&mut self, keyword: Keyword) -> TokenResult<Keyword> {
    self.skip_noop();

    if self.peek_next_n(keyword.str().len()) == Some(keyword.str()) {
      // ensure the following character is not a valid identifier character
      if let Some(after_char) = self
        .peek_next_n(keyword.str().len() + 1)
        .and_then(|str| str.chars().nth(0))
      {
        if Identifier::is_valid_char(after_char) {
          return Err(TokenError("invalid keyword"));
        }
      }

      self.consume_next_n(keyword.str().len());
      Ok(keyword)
    } else {
      Err(TokenError("expected keyword"))
    }
  }
}
