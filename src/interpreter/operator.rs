use super::RuntimeError;
use super::{scope::ScopeStack, variable::Variable, RuntimeResult};
use crate::ast::{identifier, Location};
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
      Operator::Assign => match right {
        Expression::Identifier(identifier, _) => {
          scope.set(right.try_into_identifier()?, left);
          return Ok(Variable::Nil);
        }
        Expression::Index(identifier, expression, _) => {
          let location = right.location();
          let idx = expression.eval(scope)?.try_into_number(location)?;
          let list = scope.get(identifier, location)?.try_into_list(location)?;
          if (idx < 0.0) || (idx as usize >= list.len()) {
            return Err(RuntimeError::IndexOutOfBounds {
              index: idx as usize,
              length: list.len(),
              location,
            });
          }
          // this would be REALLY slow but I can't figure out how to get a mutable reference to the list
          let mut clonedList = list.clone();
          clonedList[idx as usize] = left;

          scope.set(*identifier, Variable::List(clonedList));
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
