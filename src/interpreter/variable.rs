use std::collections::HashMap;

use super::{RuntimeError, RuntimeResult};
use crate::ast::{statement::function::Function, Location};

#[derive(Clone, Debug)]
pub enum Variable<'a> {
  String(String),
  Number(f64),
  Bool(bool),
  Function(Function<'a>),
  NativeFunction(fn(Vec<Variable<'a>>) -> Variable<'a>),
  List(Vec<Variable<'a>>),
  Map(HashMap<String, Variable<'a>>),
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
      Variable::Map(_) => write!(f, "<map>"),
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

  pub fn get_at_index(&self, index: Variable<'a>, location: Location) -> RuntimeResult<&Variable<'a>> {
    match self {
      Variable::List(list) => {
        let index: usize = index.try_into_number(location)? as usize;
        list.get(index).ok_or(RuntimeError::IndexOutOfBounds {
          index,
          length: list.len(),
          location: location,
        })
      }
      Variable::Map(values) => {
        let key = index.try_into_str(location)?;
        values.get(key).ok_or_else(|| RuntimeError::KeyNotFound {
          key: key.to_owned(),
          location: location,
        })
      }
      _ => Err(RuntimeError::InvalidType {
        expected: "list or map",
        location: location,
      }),
    }
  }

  pub fn get_at_index_mut(&mut self, index: Variable<'a>, location: Location) -> RuntimeResult<&mut Variable<'a>> {
    match self {
      Variable::List(list) => {
        let length = list.len();
        let index: usize = index.try_into_number(location)? as usize;
        list.get_mut(index).ok_or(RuntimeError::IndexOutOfBounds {
          index,
          length,
          location: location,
        })
      }
      Variable::Map(values) => {
        let key = index.try_into_str(location)?;
        values.get_mut(key).ok_or_else(|| RuntimeError::KeyNotFound {
          key: key.to_owned(),
          location: location,
        })
      }
      _ => Err(RuntimeError::InvalidType {
        expected: "list or map",
        location: location,
      }),
    }
  }

  pub fn set_at_index(&mut self, index: Variable<'a>, value: Variable<'a>, location: Location) -> RuntimeResult<()> {
    match self {
      Variable::List(list) => {
        let index = index.try_into_number(location)? as usize;
        if index >= list.len() {
          return Err(RuntimeError::IndexOutOfBounds {
            index: index as usize,
            length: list.len(),
            location: location,
          });
        }
        list[index as usize] = value;
        Ok(())
      }
      Variable::Map(map) => {
        let key = index.try_into_str(location)?;
        map.insert(key.to_owned(), value);
        Ok(())
      }
      _ => Err(RuntimeError::InvalidType {
        expected: "list or map",
        location: location,
      }),
    }
  }
}
