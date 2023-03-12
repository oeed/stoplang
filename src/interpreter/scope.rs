use std::collections::HashMap;

use crate::ast::identifier::Identifier;

use super::{variable::Variable, RuntimeError, RuntimeResult};

struct Scope<'a> {
  pub variables: HashMap<Identifier<'a>, Variable<'a>>,
}

impl<'a> Scope<'a> {
  fn new() -> Self {
    Scope {
      variables: HashMap::new(),
    }
  }

  fn get(&self, name: &Identifier<'a>) -> Option<&Variable<'a>> {
    self.variables.get(&name)
  }

  fn get_mut(&mut self, name: &Identifier<'a>) -> Option<&mut Variable<'a>> {
    self.variables.get_mut(&name)
  }

  fn set(&mut self, name: Identifier<'a>, variable: Variable<'a>) {
    self.variables.insert(name, variable);
  }
}

pub struct ScopeStack<'a>(Vec<Scope<'a>>);

impl<'a> ScopeStack<'a> {
  pub fn new() -> Self {
    ScopeStack(vec![Scope::new()])
  }

  pub fn get(&self, name: &Identifier<'a>) -> RuntimeResult<&Variable<'a>> {
    for scope in self.0.iter().rev() {
      if let Some(var) = scope.get(name) {
        return Ok(var);
      }
    }

    Err(RuntimeError::UnknownVariable {
      name: name.0.to_string(),
    })
  }

  pub fn get_mut(&mut self, name: &Identifier<'a>) -> RuntimeResult<&mut Variable<'a>> {
    for scope in self.0.iter_mut().rev() {
      if let Some(var) = scope.get_mut(name) {
        return Ok(var);
      }
    }

    Err(RuntimeError::UnknownVariable {
      name: name.0.to_string(),
    })
  }

  pub fn set(&mut self, name: Identifier<'a>, variable: Variable<'a>) {
    self.0.last_mut().unwrap().set(name, variable);
  }

  pub fn push(&mut self) {
    self.0.push(Scope::new());
  }

  pub fn pop(&mut self) {
    self.0.pop();
  }
}
