use crate::token::{Grammar, Keyword, TokenStream};

use self::conditional::Conditional;

use super::{expression::Expression, identifier::Identifier, AstError, AstResult};

pub mod conditional;
pub mod function;

pub enum Statement<'a> {
  Assignment {
    variable: Identifier<'a>,
    value: Expression<'a>,
  },
  Expression(Expression<'a>),
  Conditional(Conditional<'a>),
}

impl<'a> Statement<'a> {
  pub fn try_statement_opt(tokens: &mut TokenStream<'a>) -> AstResult<Option<Self>> {
    tokens.skip_noop();
    if tokens.is_empty() {
      return Ok(None);
    }

    if let Some(conditional) = Conditional::try_conditional_opt(tokens)? {
      Ok(Some(Statement::Conditional(conditional)))
    } else {
      Ok(Some(Statement::Expression(Expression::try_expression(tokens)?)))
    }
  }

  pub fn try_block(tokens: &mut TokenStream<'a>) -> AstResult<Vec<Self>> {
    tokens.try_delimiter(Grammar::CloseBracket)?;
    let mut statements = Vec::new();
    loop {
      if tokens.try_delimiter(Grammar::OpenBracket).is_ok() {
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
