use crate::ast::{
  expression::{self, Expression},
  identifier,
  statement::Statement,
  Location,
};

use super::{scope::ScopeStack, statement::StatementValue, variable::Variable, Eval, RuntimeError, RuntimeResult};

impl<'a> Expression<'a> {
  pub fn eval(&self, scope: &mut ScopeStack<'a>) -> RuntimeResult<Variable<'a>> {
    match self {
      Expression::Bool(bool, _) => Ok(Variable::Bool(*bool)),
      Expression::String(str, _) => Ok(Variable::String(str.to_string())),
      Expression::Number(num, _) => Ok(Variable::Number(*num)),
      Expression::Identifier(name, location) => Ok(scope.get(name, *location)?.clone()), // variables are always copied
      Expression::Brackets(expr, location) => expr.eval(scope),
      Expression::Operation {
        operator,
        left,
        right,
        location,
      } => operator.eval(scope, *location, left, right),
      Expression::Call {
        function,
        arguments,
        location,
      } => {
        let variable = scope.get(function, *location)?.clone();

        match variable {
          Variable::Function(function) => {
            if arguments.len() != function.arguments.len() {
              return Err(RuntimeError::IncorrectArgumentCount {
                function_name: function.name.to_string(),
                expected: function.arguments.len(),
                received: arguments.len(),
                location: *location,
              });
            }
            scope.push();
            for (i, provided) in arguments.iter().enumerate() {
              let expected = function.arguments[i];
              let value = provided.eval(scope)?;
              scope.set(expected, value);
            }

            let result = match Statement::eval_block(scope, &function.block)? {
              StatementValue::Early(value) | StatementValue::End(value) => value,
            };
            scope.pop();
            Ok(result)
          }
          Variable::NativeFunction(native_fn) => {
            let args = arguments
              .iter()
              .map(|expr| expr.eval(scope))
              .collect::<RuntimeResult<Vec<_>>>()?;
            Ok(native_fn(args))
          }
          _ => {
            return Err(RuntimeError::InvalidType {
              expected: "function",
              location: *location,
            })
          }
        }
      }
      Expression::List(expr, location) => {
        let mut list = Vec::new();
        for (value) in expr {
          let variable = value.eval(scope)?;
          list.push(variable);
        }
        Ok(Variable::List(list))
      }
      Expression::Index(identifier, expression, location) => {
        // The expression must evaluate to a number
        let idx = expression.eval(scope)?.try_into_number(*location)?;
        let list = scope.get(identifier, *location)?.try_into_list(*location)?;
        if idx < 0.0 || idx as usize >= list.len() {
          return Err(RuntimeError::IndexOutOfBounds {
            index: idx as usize,
            length: list.len(),
            location: *location,
          });
        }
        Ok(list[idx as usize].clone())
      }
    }
  }
}
