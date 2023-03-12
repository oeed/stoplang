use std::collections::HashMap;

use crate::ast::identifier::Identifier;

use super::{variable::Variable, RuntimeError, RuntimeResult};

pub struct Scope<'a> {
  pub variables: HashMap<Identifier<'a>, Variable<'a>>,
}

impl<'a> Scope<'a> {
  pub fn new() -> Self {
    Scope {
      variables: HashMap::new(),
    }
  }

  pub fn get(&self, name: &Identifier<'a>) -> RuntimeResult<&Variable<'a>> {
    self.variables.get(&name).ok_or_else(|| RuntimeError::UnknownVariable {
      name: name.0.to_string(),
    })
  }

  pub fn get_mut(&mut self, name: &Identifier<'a>) -> RuntimeResult<&mut Variable<'a>> {
    self
      .variables
      .get_mut(&name)
      .ok_or_else(|| RuntimeError::UnknownVariable {
        name: name.0.to_string(),
      })
  }

  pub fn set(&mut self, name: Identifier<'a>, variable: Variable<'a>) {
    self.variables.insert(name, variable);
  }
}
