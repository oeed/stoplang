use crate::ast::{expression::Expression, statement::Statement, Location};

use super::{scope::ScopeStack, stopstd::std_call, variable::Variable, Eval, RuntimeError, RuntimeResult};

impl<'a> Eval<'a> for Expression<'a> {
  fn eval(&self, scope: &mut ScopeStack<'a>, _: Location) -> RuntimeResult<Variable<'a>> {
    match self {
      Expression::Bool(bool, _) => Ok(Variable::Bool(*bool)),
      Expression::String(str, _) => Ok(Variable::String(str.to_string())),
      Expression::Number(num, _) => Ok(Variable::Number(*num)),
      Expression::Identifier(name, location) => Ok(scope.get(name, *location)?.clone()), // variables are always copied
      Expression::Brackets(expr, location) => expr.eval(scope, *location),
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
        if let Some(std_value) = std_call(*function, scope, arguments)? {
          return Ok(std_value);
        }

        let function = scope.get(function, *location)?.try_into_function(*location)?.clone();
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
          let value = provided.eval(scope, provided.location())?;
          scope.set(expected, value);
        }

        // function.eval(&mut function_scope)
        let result = Statement::eval_block(scope, &function.block)?;
        scope.pop();
        Ok(result)
      }
    }
  }
}
