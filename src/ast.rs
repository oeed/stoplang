use thiserror::Error;

use self::statement::Statement;
use crate::token::{TokenError, TokenStream};

pub mod expression;
pub mod identifier;
pub mod statement;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AstError {
  #[error(transparent)]
  TokenError(#[from] TokenError),
  #[error("missing expression")]
  MissingExpression(Location),
  #[error("missing statement")]
  MissingStatement(Location),
  #[error("missing identifier")]
  MissingIdentifier(Location),
}
pub type AstResult<T> = Result<T, AstError>;

impl AstError {
  pub fn location(&self) -> Location {
    match self {
      AstError::TokenError(TokenError { location, .. })
      | AstError::MissingExpression(location)
      | AstError::MissingStatement(location)
      | AstError::MissingIdentifier(location) => *location,
    }
  }
}

#[derive(Debug)]
pub struct Ast<'a> {
  pub statements: Vec<Statement<'a>>,
}

impl<'a> Ast<'a> {
  pub fn new(tokens: &mut TokenStream<'a>) -> AstResult<Self> {
    let mut statements = Vec::new();
    loop {
      if let Some(statement) = Statement::try_statement_opt(tokens)? {
        statements.push(statement)
      }
      else {
        break;
      }
    }
    Ok(Ast { statements })
  }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Location {
  /// `None` if end of file
  pub position: Option<usize>,
}

impl Location {
  pub fn new(position: Option<usize>) -> Self {
    Location { position }
  }

  pub fn description(&self, file: &str) -> String {
    if let Some(position) = self.position {
      let mut n = 0;
      for (l, line) in file.lines().enumerate() {
        for (c, _) in line.chars().chain(std::iter::once('\n')).enumerate() {
          n += 1; // increment 1 for line ending
          if n == position {
            return format!("line {}, col {}", l + 1, c + 1);
          }
        }
      }
      format!("line {}, col {}", 0, 0)
    }
    else {
      String::from("end of file")
    }
  }
}
