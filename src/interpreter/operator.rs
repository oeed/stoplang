use super::{scope::ScopeStack, variable::Variable, RuntimeResult};
use crate::ast::Location;
use crate::interpreter::Eval;
use crate::{ast::expression::Expression, token::Operator};

impl Operator {
  pub fn eval<'a>(
    &self,
    scope: &mut ScopeStack<'a>,
    _: Location,
    left: &Expression<'a>,
    right: &Expression<'a>,
  ) -> RuntimeResult<Variable<'a>> {
    let left_loc = left.location();
    let left = left.eval(scope)?;
    match self {
      Operator::Assign => {
        scope.set(right.try_into_identifier()?, left);
        return Ok(Variable::Nil);
      }
      _ => (),
    }

    let right_loc = right.location();
    let right = right.eval(scope)?;
    match self {
      Operator::Equals => Ok(Variable::Bool(left == right)),
      Operator::Divide => Ok(Variable::Number(
        right.try_into_number(right_loc)? / left.try_into_number(left_loc)?,
      )),
      Operator::Multiply => Ok(Variable::Number(
        right.try_into_number(right_loc)? * left.try_into_number(left_loc)?,
      )),
      Operator::Add => Ok(Variable::Number(
        right.try_into_number(right_loc)? + left.try_into_number(left_loc)?,
      )),
      Operator::Subtract => Ok(Variable::Number(
        right.try_into_number(right_loc)? - left.try_into_number(left_loc)?,
      )),
      Operator::Modulo => Ok(Variable::Number(
        right.try_into_number(right_loc)? % left.try_into_number(left_loc)?,
      )),
      Operator::Lte => Ok(Variable::Bool(
        left.try_into_number(left_loc)? <= right.try_into_number(right_loc)?,
      )),
      Operator::Gte => Ok(Variable::Bool(
        left.try_into_number(left_loc)? >= right.try_into_number(right_loc)?,
      )),
      Operator::Lt => Ok(Variable::Bool(
        left.try_into_number(left_loc)? < right.try_into_number(right_loc)?,
      )),
      Operator::Gt => Ok(Variable::Bool(
        left.try_into_number(left_loc)? > right.try_into_number(right_loc)?,
      )),
      Operator::And => Ok(Variable::Bool(
        left.try_into_bool(left_loc)? && right.try_into_bool(right_loc)?,
      )),
      Operator::Or => Ok(Variable::Bool(
        left.try_into_bool(left_loc)? || right.try_into_bool(right_loc)?,
      )),
      Operator::Assign => unreachable!(),
    }
  }
}
