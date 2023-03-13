use crate::ast::{statement::function::Function, Location};
use derive_more::Display;

use super::{scope::ScopeStack, RuntimeError, RuntimeResult};

#[derive(Clone)]
pub enum Variable<'a> {
  String(String),
  Number(f64),
  Bool(bool),
  Function(Function<'a>),
  NativeFunction(fn(Vec<Variable<'a>>) -> Variable<'a>),
  List(Vec<Variable<'a>>),
  Nil,
}

impl<'a> PartialEq for Variable<'a> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Variable::String(string), Variable::String(other_string)) => string == other_string,
      (Variable::Number(number), Variable::Number(other_number)) => number == other_number,
      (Variable::Bool(bool), Variable::Bool(other_bool)) => bool == other_bool,
      (Variable::Function(func), Variable::Function(other_func)) => func == other_func,
      (Variable::List(list), Variable::List(other_list)) => list == other_list,
      (Variable::Nil, Variable::Nil) => true,
      _ => false,
    }
  }
}

impl<'a> std::fmt::Display for Variable<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Variable::String(string) => write!(f, "{}", string),
      Variable::Number(number) => write!(f, "{}", number),
      Variable::Bool(bool) => write!(f, "{}", bool),
      Variable::Function(func) => write!(f, "{}", func),
      Variable::List(list) => {
        write!(f, "[")?;
        for (i, item) in list.iter().enumerate() {
          write!(f, "{}", item)?;
          if i != list.len() - 1 {
            write!(f, ", ")?;
          }
        }
        write!(f, "]")
      }
      Variable::Nil => write!(f, "nil"),
      Variable::NativeFunction(_) => write!(f, "<native function>"),
    }
  }
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

  pub fn try_into_list(&self, location: Location) -> RuntimeResult<&Vec<Variable<'a>>> {
    match self {
      Variable::List(list) => Ok(list),
      _ => Err(RuntimeError::InvalidType {
        expected: "list",
        location,
      }),
    }
  }
}
