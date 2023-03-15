use super::{scope::ScopeStack, variable::Variable, RuntimeResult};
use crate::ast::{
  statement::{conditional::Conditional, Statement},
  Location,
};

impl<'a> Statement<'a> {
  fn eval(&self, scope: &mut ScopeStack<'a>) -> RuntimeResult<StatementValue<'a>> {
    match self {
      Statement::Conditional(conditional) => conditional.eval(scope),
      Statement::Expression(expression) => Ok(StatementValue::End(expression.eval(scope)?)),
      Statement::Function(function) => {
        scope.set(function.name.clone(), Variable::Function(function.clone()));
        Ok(StatementValue::End(Variable::Nil))
      }
      Statement::While(while_loop) => {
        while while_loop
          .condition
          .eval(scope)?
          .try_into_bool(while_loop.condition.location())?
        {
          match Statement::eval_block(scope, &while_loop.block)? {
            StatementValue::Early(value) => return Ok(StatementValue::Early(value)),
            _ => (),
          }
        }
        Ok(StatementValue::End(Variable::Nil))
      }
      // this path only happens if it's the last statement, so it's fine anyway
      Statement::Return(expression) => Ok(StatementValue::Early(expression.eval(scope)?)),
    }
  }
}

pub enum StatementValue<'a> {
  Early(Variable<'a>),
  End(Variable<'a>),
}

impl<'a> Statement<'a> {
  pub fn location(&self) -> Location {
    match self {
      Statement::Conditional(conditional) => conditional.location,
      Statement::Expression(expression) => expression.location(),
      Statement::Function(function) => function.location,
      Statement::While(while_loop) => while_loop.location,
      Statement::Return(ret) => ret.location(),
    }
  }

  pub fn eval_block(scope: &mut ScopeStack<'a>, block: &[Statement<'a>]) -> RuntimeResult<StatementValue<'a>> {
    let mut statements = block.iter().rev();
    let last_statement = statements.next();
    for statement in statements.rev() {
      match statement {
        Statement::Return(expression) => return Ok(StatementValue::Early(expression.eval(scope)?)),
        statement => match statement.eval(scope)? {
          StatementValue::Early(value) => return Ok(StatementValue::Early(value)),
          _ => (),
        },
      }
    }

    if let Some(last_statement) = last_statement {
      Ok(last_statement.eval(scope)?)
    } else {
      Ok(StatementValue::End(Variable::Nil))
    }
  }
}

impl<'a> Conditional<'a> {
  fn eval(&self, scope: &mut ScopeStack<'a>) -> RuntimeResult<StatementValue<'a>> {
    let condition = self.condition.eval(scope)?.try_into_bool(self.condition.location())?;
    if condition {
      Statement::eval_block(scope, &self.true_block)
    } else {
      Statement::eval_block(scope, &self.false_block)
    }
  }
}
