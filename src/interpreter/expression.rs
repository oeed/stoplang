use super::{scope::ScopeStack, statement::StatementValue, variable::Variable, RuntimeError, RuntimeResult};
use crate::ast::{expression::Expression, statement::Statement};

impl<'a> Expression<'a> {
  pub fn eval(&self, scope: &mut ScopeStack<'a>) -> RuntimeResult<Variable<'a>> {
    match self {
      Expression::Bool(bool, _) => Ok(Variable::Bool(*bool)),
      Expression::String(str, _) => Ok(Variable::String(str.to_string())),
      Expression::Number(num, _) => Ok(Variable::Number(*num)),
      Expression::Identifier(name, location) => Ok(scope.get(name, *location)?.clone()), // variables are always copied
      Expression::Brackets(expr, ..) => expr.eval(scope),
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
              let expected = &function.arguments[i];
              let value = provided.eval(scope)?;
              scope.set(expected.clone(), value);
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
          _ => Err(RuntimeError::InvalidType {
            expected: "function",
            location: *location,
          }),
        }
      }
      Expression::List(expr, ..) => {
        let mut list = Vec::new();
        for value in expr {
          let variable = value.eval(scope)?;
          list.push(variable);
        }
        Ok(Variable::List(list))
      }
      Expression::Index {
        indexed,
        indices,
        location,
      } => {
        // Index expressions are only valid for lists and maps
        let mut variable = scope.get(indexed, *location)?.clone();

        for value in indices {
          let idx = value.eval(scope)?;
          let result = variable.get_at_index(idx, *location)?;
          variable = result;
        }
        Ok(variable)
      }
      Expression::Map(values, ..) => {
        let mut map = std::collections::HashMap::new();

        for (ident, expr) in values {
          let key = ident.clone();
          let value = expr.eval(scope)?;
          map.insert(key, value);
        }

        Ok(Variable::Map(map))
      }
    }
  }
}
