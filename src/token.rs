use crate::ast::{identifier::Identifier, Location};
use thiserror::Error;

#[derive(PartialEq, Eq, Error, Debug)]
#[error("token error: {error}")]
pub struct TokenError {
  error: String,
  pub location: Location,
}
pub type TokenResult<T> = Result<T, TokenError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grammar {
  OpenBracket,
  CloseBracket,
  OpenCurly,
  CloseCurly,
  DoubleQuote,
  Comma,
}

impl Grammar {
  fn str(&self) -> &'static str {
    use Grammar::*;
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
  Equals,
  Divide,
  Multiply,
  Add,
  Subtract,
  Lte,
  Gte,
  Lt,
  Gt,
  And,
  Or,
  Assign,
}

impl Operator {
  fn str(&self) -> &'static str {
    use Operator::*;
    match self {
      Equals => "==",
      Divide => "/",
      Multiply => "*",
      Add => "+",
      Subtract => "-",
      Lte => "<=",
      Gte => ">=",
      Lt => "<",
      Gt => ">",
      And => "&&",
      Or => "||",
      Assign => "=",
    }
  }

  pub fn operators() -> &'static [Operator] {
    use Operator::*;
    &[
      Equals, Divide, Multiply, Add, Subtract, Lte, Gte, Lt, Gt, And, Or, Assign,
    ]
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
  If,
  Else,
  Fn,
  True,
  False,
  Return,
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
      Return => "return",
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

  pub fn location(&self) -> Location {
    Location {
      position: self.next_position,
    }
  }

  /// Peek at the next `n` character, if there are not `n` many characters left returns `None`
  fn peek_next_n(&self, n: usize) -> Option<&'a str> {
    self.next_position.and_then(|next_pos| {
      if n <= next_pos + 1 {
        self.string.get(next_pos + 1 - n..=next_pos)
      } else {
        None
      }
    })
  }

  fn consume_next_n(&mut self, n: usize) -> Option<&'a str> {
    if let Some(next_pos) = self.next_position {
      if n <= next_pos + 1 {
        let char = self
          .string
          .get(next_pos + 1 - n..=next_pos)
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
    } else {
      None
    }
  }

  fn peek_next_char(&self) -> Option<char> {
    self.peek_next_n(1).and_then(|str| str.chars().nth(0))
  }

  fn consume_next_char(&mut self) -> Option<char> {
    self.consume_next_n(1).and_then(|str| str.chars().nth(0))
  }

  // Skip any comments or whitespace
  pub fn skip_noop(&mut self) {
    loop {
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
        } else {
          break;
        }
      } else {
        break;
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
          return Err(TokenError {
            error: format!("invalid first character '{char}' of identifier, must only be alphabetic or _",),
            location: self.location(),
          });
        }
      } else if !Identifier::is_valid_char(char) {
        // end of identifier
        return Ok(Some(Identifier(self.consume_next_n(n - 1).unwrap())));
      }
    }
    unreachable!()
  }

  pub fn try_identifier(&mut self) -> TokenResult<Identifier<'a>> {
    self.try_identifier_opt()?.ok_or_else(|| TokenError {
      error: "missing identifier".to_string(),
      location: self.location(),
    })
  }

  pub fn try_number_opt(&mut self) -> TokenResult<Option<f64>> {
    self.skip_noop();
    let mut had_decimal = false; // whether a decimal has already been seen
    for n in 1.. {
      match self.peek_next_n(n) {
        Some(str) => {
          let char = str.chars().nth(0).unwrap();
          if n == 1 && !char.is_numeric() {
            return Ok(None);
          } else if char.is_numeric() {
            continue;
          } else if char == '.' {
            if n == 1 {
              return Err(TokenError {
                error: "number cannot end in decimal".to_string(),
                location: self.location(),
              });
            } else if had_decimal {
              return Err(TokenError {
                error: "invalid number, cannot have multiple decimals".to_string(),
                location: self.location(),
              });
            } else {
              had_decimal = true;
            }
            continue;
          } else {
            // non-number char, thus end of number
          }
        }
        None if n == 1 => return Ok(None),
        None => (), // end of file, thus end of number
      };

      // end of number
      let number_str = self.consume_next_n(n - 1).unwrap();
      return Ok(Some(
        number_str
          .parse()
          .expect("number parsing restrictions should result in valid float"),
      ));
    }
    unreachable!()
  }

  pub fn try_string_opt(&mut self) -> TokenResult<Option<&'a str>> {
    self.skip_noop();

    if self.try_chars(Grammar::DoubleQuote.str()).is_err() {
      return Ok(None);
    }
    for n in 1.. {
      let str = self.peek_next_n(n).ok_or_else(|| TokenError {
        error: "expected string, found nothing".to_string(),
        location: self.location(),
      })?;

      // TODO: unsure what an unfinished string will do here
      let char = str.chars().nth(0).unwrap();
      if char == Grammar::DoubleQuote.str().chars().next().unwrap() {
        let inner_str = self.consume_next_n(n - 1).unwrap();
        self.consume_next_char(); // consume the trailing "
        return Ok(Some(inner_str));
      }
    }
    unreachable!()
  }

  fn try_chars<'b>(&mut self, str: &'b str) -> TokenResult<&'b str> {
    self.skip_noop();

    if self.peek_next_n(str.len()) == Some(str) {
      self.consume_next_n(str.len());
      Ok(str)
    } else {
      Err(TokenError {
        error: format!("expected: {str}"),
        location: self.location(),
      })
    }
  }

  pub fn try_operator(&mut self, operator: Operator) -> TokenResult<Operator> {
    self.try_chars(operator.str()).map(|_| operator)
  }

  pub fn try_grammar(&mut self, grammar: Grammar) -> TokenResult<Grammar> {
    self.try_chars(grammar.str()).map(|_| grammar)
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
          return Err(TokenError {
            error: format!("invalid keyword: {after_char}{}", keyword.str()),
            location: self.location(),
          });
        }
      }

      self.consume_next_n(keyword.str().len());
      Ok(keyword)
    } else {
      Err(TokenError {
        error: format!("expected keyword '{}'", keyword.str()),
        location: self.location(),
      })
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn keyword() {
    let mut tokens = TokenStream::new(" if ");
    assert_eq!(tokens.try_keyword(Keyword::If), Ok(Keyword::If));
  }

  #[test]
  fn grammar() {
    let mut tokens = TokenStream::new(" () ");
    assert!(tokens.try_grammar(Grammar::CloseCurly).is_err(),);
    assert_eq!(tokens.try_grammar(Grammar::CloseBracket), Ok(Grammar::CloseBracket));
    assert_eq!(tokens.try_grammar(Grammar::OpenBracket), Ok(Grammar::OpenBracket));
  }

  #[test]
  fn operator() {
    let mut tokens = TokenStream::new("+ == ");
    assert_eq!(tokens.try_operator(Operator::Equals), Ok(Operator::Equals));
    assert_eq!(tokens.try_operator(Operator::Add), Ok(Operator::Add));
  }

  #[test]
  fn string_opt() {
    let mut tokens = TokenStream::new(" \"hello there\" ");
    assert_eq!(tokens.try_string_opt(), Ok(Some("hello there")));
    assert_eq!(tokens.try_string_opt(), Ok(None));
  }

  #[test]
  fn number_opt() {
    let mut tokens = TokenStream::new(" 1 33.01 55 ");
    assert_eq!(tokens.try_number_opt(), Ok(Some(55.)));
    assert_eq!(tokens.try_number_opt(), Ok(Some(33.01)));
    assert_eq!(tokens.try_number_opt(), Ok(Some(1.)));
    assert_eq!(tokens.try_number_opt(), Ok(None));
  }

  #[test]
  fn identifier_opt() {
    let mut tokens = TokenStream::new("  2mY_var ");
    assert_eq!(tokens.try_identifier_opt(), Ok(Some(Identifier("2mY_var"))));
    assert_eq!(tokens.try_identifier_opt(), Ok(Some(Identifier("arg1"))));
    assert_eq!(tokens.try_identifier_opt(), Ok(None));
  }

  #[test]
  fn identifier_invalid() {
    let mut tokens = TokenStream::new(" var2 ");
    assert!(tokens.try_identifier_opt().is_err());
  }

  #[test]
  fn skip_noop() {
    let mut tokens = TokenStream::new(
      " blah \\\\
    
      blah again\\\\

       ",
    );
    assert!(!tokens.is_empty());
    tokens.skip_noop();
    assert!(tokens.is_empty());
  }
}
