use crate::token::{TokenError, TokenStream};
use thiserror::Error;

use self::statement::Statement;

pub mod expression;
pub mod identifier;
pub mod statement;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AstError {
  #[error(transparent)]
  TokenError(#[from] TokenError),
  #[error("missing expression")]
  MissingExpression,
  #[error("missing statement")]
  MissingStatement,
  #[error("missing identifier")]
  MissingIdentifier,
}
pub type AstResult<T> = Result<T, AstError>;

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
      } else {
        break;
      }
    }

    Ok(Ast { statements })
  }
}
