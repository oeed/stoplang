use crate::ast::{statement::function::Function, Location};
use derive_more::Display;

use super::{RuntimeError, RuntimeResult};

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Variable<'a> {
  String(String),
  Number(f64),
  Bool(bool),
  Function(Function<'a>),
  Nil,
}

impl<'a> Variable<'a> {
  pub fn try_into_function(&self, location: Location) -> RuntimeResult<&Function<'a>> {
    match self {
      Variable::Function(func) => Ok(func),
      _ => Err(RuntimeError::InvalidType {
        expected: "function",
        location,
      }),
    }
  }

  pub fn try_into_bool(&self, location: Location) -> RuntimeResult<bool> {
    match self {
      Variable::Bool(bool) => Ok(*bool),
      _ => Err(RuntimeError::InvalidType {
        expected: "bool",
        location,
      }),
    }
  }

  pub fn try_into_number(&self, location: Location) -> RuntimeResult<f64> {
    match self {
      Variable::Number(number) => Ok(*number),
      _ => Err(RuntimeError::InvalidType {
        expected: "number",
        location,
      }),
    }
  }

  pub fn try_into_str(&self, location: Location) -> RuntimeResult<&str> {
    match self {
      Variable::String(string) => Ok(string),
      _ => Err(RuntimeError::InvalidType {
        expected: "string",
        location,
      }),
    }
  }
}
