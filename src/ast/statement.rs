use crate::token::{Grammar, Keyword, TokenStream};

use self::{conditional::Conditional, function::Function};

use super::{expression::Expression, AstError, AstResult};

pub mod conditional;
pub mod function;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement<'a> {
  Conditional(Conditional<'a>),
  Expression(Expression<'a>),
  Function(Function<'a>),
  Return(Expression<'a>),
}

impl<'a> Statement<'a> {
  pub fn try_statement_opt(tokens: &mut TokenStream<'a>) -> AstResult<Option<Self>> {
    tokens.skip_noop();
    if tokens.is_empty() {
      return Ok(None);
    }

    let statement = if tokens.try_keyword(Keyword::Return).is_ok() {
      Statement::Return(Expression::try_expression(tokens)?)
    } else if let Some(conditional) = Conditional::try_conditional_opt(tokens)? {
      Statement::Conditional(conditional)
    } else if let Some(function) = Function::try_function_opt(tokens)? {
      Statement::Function(function)
    } else {
      Statement::Expression(Expression::try_expression(tokens)?)
    };

    Ok(Some(statement))
  }

  pub fn try_block(tokens: &mut TokenStream<'a>) -> AstResult<Vec<Self>> {
    tokens.try_grammar(Grammar::CloseCurly)?;
    let mut statements = Vec::new();
    loop {
      if tokens.try_grammar(Grammar::OpenCurly).is_ok() {
        break;
      }
      if let Some(statement) = Statement::try_statement_opt(tokens)? {
        statements.push(statement)
      } else {
        return Err(AstError::MissingStatement);
      }
    }

    Ok(statements)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn block() {
    let mut tokens = TokenStream::new(
      "{
        false
        true
      }",
    );
    assert_eq!(
      Statement::try_block(&mut tokens),
      Ok(vec![
        Statement::Expression(Expression::Bool(true)),
        Statement::Expression(Expression::Bool(false))
      ])
    );
  }

  #[test]
  fn none_statement() {
    let mut tokens = TokenStream::new(
      "
      comment \\
      
      ",
    );
    assert_eq!(Statement::try_statement_opt(&mut tokens), Ok(None));
  }
}
