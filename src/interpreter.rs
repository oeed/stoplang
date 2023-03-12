use crate::ast::{statement::Statement, Ast};
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
  UnknownVariable { name: String },
  #[error("invalid type, expected type {expected}")]
  InvalidType { expected: &'static str },
  #[error("invalid expression, expected {expected}")]
  InvalidExpression { expected: &'static str },
  #[error("invalid number of arguments in call to '{function_name}', received: {received}, expected: {expected}")]
  IncorrectArgumentCount {
    function_name: String,
    expected: usize,
    received: usize,
  },
}
pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub fn interpret(ast: Ast<'_>) -> RuntimeResult<()> {
  let mut scope = ScopeStack::new();
  Statement::eval_block(&mut scope, &ast.statements)?;
  Ok(())
}

trait Eval<'a> {
  fn eval(&self, scope: &mut ScopeStack<'a>) -> RuntimeResult<Variable<'a>>;
}
