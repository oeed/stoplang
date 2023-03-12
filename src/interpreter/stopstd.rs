//! Stop's 'standard library' functions

use crate::ast::{expression::Expression, identifier::Identifier};

use super::{scope::ScopeStack, variable::Variable, Eval, RuntimeResult};

/// Returns `Some` if it matched and called a standard library function, `None` if it didn't
pub fn std_call<'a>(
  identifier: Identifier<'a>,
  scope: &mut ScopeStack<'a>,
  arguments: &[Expression<'a>],
) -> RuntimeResult<Option<Variable<'static>>> {
  match identifier {
    Identifier("print") => Ok(Some(print(eval_arguments(scope, arguments)?))),
    _ => Ok(None),
  }
}

fn eval_arguments<'a>(scope: &mut ScopeStack<'a>, arguments: &[Expression<'a>]) -> RuntimeResult<Vec<Variable<'a>>> {
  arguments.iter().map(|expr| expr.eval(scope)).collect()
}

fn print(arguments: Vec<Variable<'_>>) -> Variable<'static> {
  for argument in arguments {
    println!("{}", argument)
  }
  Variable::Nil
}
