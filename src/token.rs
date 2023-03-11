use crate::ast::identifier::Identifier;

struct TokenError(&'static str);
type TokenResult<T> = Result<T, TokenError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
  OpenBracket,
  CloseBracket,
  OpenCurly,
  CloseCurly,
  Equals,
  Divide,
  Multiply,
  Add,
  Subtract,
}

impl Operator {
  fn str(&self) -> &'static str {
    use Operator::*;
    match self {
      OpenBracket => "(",
      CloseBracket => ")",
      OpenCurly => "{",
      CloseCurly => "}",
      Equals => "=",
      Divide => "*",
      Multiply => "/",
      Add => "-",
      Subtract => "+",
    }
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

struct TokenStream<'a> {
  // position of the next character, starts at the final index. `None` if at the end of the string.
  next_position: Option<usize>,
  string: &'a str,
}

impl<'a> TokenStream<'a> {
  fn new(string: &'a str) -> Self {
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
    self.next_position.and_then(|next| self.string.get(next - n..next))
  }

  fn consume_next_n(&mut self, n: usize) -> Option<&'a str> {
    if let Some(next_pos) = self.next_position {
      let char = self.string[next_pos];
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
    self.peek_next_n(1).map(|str| str[0])
  }

  fn consume_next_char(&mut self) -> Option<char> {
    self.consume_next_n(1).map(|str| str[0])
  }

  // Skip any comments or whitespace
  fn skip_inop(&mut self) {
    if let Some(next_char) = self.peek_next_char() {
      if next_char.is_whitespace() {
        // consume the whitespace, then repeat.
        self.consume_next_char();
        return self.skip_inop();
      } else if next_char == "\\" && self.peek_next_n(2) == Some("\\\\") {
        // start of a comment, read until the end of the line
        loop {
          match self.consume_next_char() {
            Some("\n") | None => break,
            _ => continue,
          }
        }
      }
    }
  }

  pub fn try_identifier(&mut self) -> TokenResult<Identifier<'a>> {
    self.skip_inop();
    for n in 1.. {
      let str = self
        .peek_next_n(n)
        .ok_or(TokenError("expected identifier, found nothing"))?;

      let char = str[0];
      if n == 1 {
        if !Identifier::is_valid_first_char(char) {
          return Err(TokenError(
            "invalid first character of identifier, must only be alphabetic or _",
          ));
        } else if !Identifier::is_valid_char(char) {
          // end of identifier
          return Ok(self.consume_next_n(n - 1).unwrap());
        }
      }
    }
  }

  pub fn try_number(&mut self) -> TokenResult<f64> {
    self.skip_inop();
    let mut had_decimal = false; // whether a decimal has already been seen
    for n in 1.. {
      let str = self
        .peek_next_n(n)
        .ok_or(TokenError("expected number, found nothing"))?;

      let char: char = str[0];
      if char.is_numeric() {
        continue;
      } else if char == "." {
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
        return Ok(
          number_str
            .parse()
            .expect("number parsing restrictions should result in valid float"),
        );
      }
    }
  }

  pub fn try_operator(&mut self, operator: Operator) -> TokenResult<Operator> {
    if self.peek_next_n(operator.str().len()) == Some(operator.str()) {
      self.consume_next_n(operator.str().len());
      Ok(operator)
    } else {
      Err(TokenError("expected operator"))
    }
  }

  pub fn try_keyword(&mut self, keyword: Keyword) -> TokenResult<Keyword> {
    if self.peek_next_n(keyword.str().len()) == Some(keyword.str()) {
      // ensure the following character is not a valid identifier character
      if let Some(after_char) = self.peek_next_n(keyword.str().len() + 1).map(|str| str[0]) {
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
