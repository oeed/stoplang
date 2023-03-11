use crate::token::TokenError;
use thiserror::Error;

use self::statement::Statement;

pub mod expression;
pub mod identifier;
pub mod statement;

#[derive(Error, Debug)]
pub enum AstError {
  #[error(transparent)]
  TokenError(#[from] TokenError),
  #[error("missing expression")]
  MissingExpression,
}
pub type AstResult<T> = Result<T, AstError>;

pub struct Ast<'a> {
  statements: Vec<Statement<'a>>,
}

impl<'a> Ast<'a> {
  pub fn new(tokens: &'a str) -> Self {}
}
