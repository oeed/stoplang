use crate::ast::{expression::Expression, statement::Statement};

use super::{scope::Scope, stopstd::std_call, variable::Variable, Eval, RuntimeError, RuntimeResult};

impl<'a> Eval<'a> for Expression<'a> {
  fn eval(&self, scope: &mut Scope<'a>) -> RuntimeResult<Variable<'a>> {
    match self {
      Expression::Bool(bool) => Ok(Variable::Bool(*bool)),
      Expression::String(str) => Ok(Variable::String(str.to_string())),
      Expression::Number(num) => Ok(Variable::Number(*num)),
      Expression::Identifier(name) => Ok(scope.get(name)?.clone()), // variables are always copied
      Expression::Brackets(expr) => expr.eval(scope),
      Expression::Operation { operator, left, right } => operator.eval(scope, left, right),
      Expression::Call { function, arguments } => {
        if let Some(std_value) = std_call(*function, scope, arguments)? {
          return Ok(std_value);
        }

        let function = scope.get(function)?.try_into_function()?.clone();
        if arguments.len() != function.arguments.len() {
          return Err(RuntimeError::IncorrectArgumentCount {
            function_name: function.name.to_string(),
            expected: function.arguments.len(),
            received: arguments.len(),
          });
        }
        let mut function_scope = Scope::new();
        for (i, provided) in arguments.iter().enumerate() {
          let expected = function.arguments[i];
          function_scope.set(expected, provided.eval(scope)?);
        }

        // function.eval(&mut function_scope)
        Statement::eval_block(&mut function_scope, &function.block)
      }
    }
  }
}
