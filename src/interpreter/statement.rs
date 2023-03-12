use crate::ast::statement::{conditional::Conditional, Statement};

use super::{scope::ScopeStack, variable::Variable, Eval, RuntimeResult};

impl<'a> Eval<'a> for Statement<'a> {
  fn eval(&self, scope: &mut ScopeStack<'a>) -> RuntimeResult<Variable<'a>> {
    match self {
      Statement::Conditional(conditional) => conditional.eval(scope),
      Statement::Expression(expression) => expression.eval(scope),
      Statement::Function(function) => {
        scope.set(function.name, Variable::Function(function.clone()));
        Ok(Variable::Nil)
      }
      Statement::Return(_) => todo!(),
    }
  }
}

impl<'a> Statement<'a> {
  pub fn eval_block(scope: &mut ScopeStack<'a>, block: &[Statement<'a>]) -> RuntimeResult<Variable<'a>> {
    let mut statements = block.iter().rev();
    let last_statement = statements.next();
    for statement in statements.rev() {
      statement.eval(scope)?;
    }

    if let Some(last_statement) = last_statement {
      Ok(last_statement.eval(scope)?)
    } else {
      Ok(Variable::Nil)
    }
  }
}

impl<'a> Eval<'a> for Conditional<'a> {
  fn eval(&self, scope: &mut ScopeStack<'a>) -> RuntimeResult<Variable<'a>> {
    let condition = self.condition.eval(scope)?.try_into_bool()?;
    if condition {
      Statement::eval_block(scope, &self.true_block)
    } else {
      Statement::eval_block(scope, &self.false_block)
    }
  }
}
