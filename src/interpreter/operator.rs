use super::{scope::Scope, variable::Variable, RuntimeResult};
use crate::interpreter::Eval;
use crate::{ast::expression::Expression, token::Operator};

impl Operator {
  pub fn eval<'a>(
    &self,
    scope: &mut Scope<'a>,
    left: &Expression<'a>,
    right: &Expression<'a>,
  ) -> RuntimeResult<Variable<'a>> {
    let left = left.eval(scope)?;
    match self {
      Operator::Assign => {
        scope.set(right.try_into_identifier()?, left);
        return Ok(Variable::Nil);
      }
      _ => (),
    }

    let right = right.eval(scope)?;
    match self {
      Operator::Equals => Ok(Variable::Bool(left == right)),
      Operator::Divide => Ok(Variable::Number(left.try_into_number()? / right.try_into_number()?)),
      Operator::Multiply => Ok(Variable::Number(left.try_into_number()? * right.try_into_number()?)),
      Operator::Add => Ok(Variable::Number(left.try_into_number()? + right.try_into_number()?)),
      Operator::Subtract => Ok(Variable::Number(left.try_into_number()? - right.try_into_number()?)),
      Operator::Lte => Ok(Variable::Bool(left.try_into_bool()? <= right.try_into_bool()?)),
      Operator::Gte => Ok(Variable::Bool(left.try_into_bool()? >= right.try_into_bool()?)),
      Operator::Lt => Ok(Variable::Bool(left.try_into_bool()? < right.try_into_bool()?)),
      Operator::Gt => Ok(Variable::Bool(left.try_into_bool()? > right.try_into_bool()?)),
      Operator::And => Ok(Variable::Bool(left.try_into_bool()? && right.try_into_bool()?)),
      Operator::Or => Ok(Variable::Bool(left.try_into_bool()? || right.try_into_bool()?)),
      Operator::Assign => unreachable!(),
    }
  }
}
