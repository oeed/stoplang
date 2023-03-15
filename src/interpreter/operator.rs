use super::{scope::ScopeStack, variable::Variable, RuntimeError, RuntimeResult};
use crate::{
  ast::{expression::Expression, Location},
  token::Operator,
};

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
      Operator::Assign => match right {
        Expression::Identifier(_, _) => {
          scope.set(right.try_into_identifier()?, left);
          return Ok(Variable::Nil);
        }
        Expression::Index(identifier, expression, _) => {
          // Only support 1 level of indexing for assignments
          // The reason for this is that I can't figure out how to get a mutable refernce (that isn't cloned) to things in a
          // nested map/list structure

          // Therefore the only solution would be to recursively clone the entire list/map hierarchy

          if expression.len() != 1 {
            return Err(RuntimeError::InvalidAssignment {
              location: right.location(),
            });
          }

          let location = right.location();
          let mut variable = scope.get(identifier, location)?.clone();
          let idx = expression[0].eval(scope)?;
          variable.set_at_index(idx, left, right.location())?;
          scope.set(identifier.clone(), variable);
          return Ok(Variable::Nil);
        }
        _ => (),
      },
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
