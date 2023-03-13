use crate::ast::{statement::Statement, Ast, Location};
use thiserror::Error;

use self::{scope::ScopeStack, variable::Variable};

mod expression;
mod operator;
mod scope;
mod statement;
mod stopstd;
mod variable;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RuntimeError {
  #[error("unknown variable '{name}'")]
  UnknownVariable { name: String, location: Location },
  #[error("invalid type, expected type {expected}")]
  InvalidType { expected: &'static str, location: Location },
  #[error("invalid expression, expected {expected}")]
  InvalidExpression { expected: &'static str, location: Location },
  #[error("invalid number of arguments in call to '{function_name}', received: {received}, expected: {expected}")]
  IncorrectArgumentCount {
    function_name: String,
    expected: usize,
    received: usize,
    location: Location,
  },
}
pub type RuntimeResult<T> = Result<T, RuntimeError>;
impl RuntimeError {
  pub fn location(&self) -> Location {
    match self {
      RuntimeError::UnknownVariable { location, .. }
      | RuntimeError::InvalidType { location, .. }
      | RuntimeError::InvalidExpression { location, .. }
      | RuntimeError::IncorrectArgumentCount { location, .. } => *location,
    }
  }
}

pub fn interpret(ast: Ast<'_>) -> RuntimeResult<()> {
  // ast.print();
  let mut scope = ScopeStack::new();
  Statement::eval_block(&mut scope, &ast.statements)?;
  Ok(())
}

trait Eval<'a> {
  fn eval(&self, scope: &mut ScopeStack<'a>, location: Location) -> RuntimeResult<Variable<'a>>;
}
