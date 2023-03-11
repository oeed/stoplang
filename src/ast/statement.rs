use crate::token::{Keyword, TokenStream};

use self::conditional::Conditional;

use super::{expression::Expression, identifier::Identifier, AstError, AstResult};

pub mod conditional;
pub mod function;

pub enum Statement<'a> {
  Assignment {
    variable: Identifier<'a>,
    value: Expression<'a>,
  },
  Conditional(Conditional<'a>),
}

impl<'a> Statement<'a> {
  pub fn try_statement_opt(tokens: &mut TokenStream<'a>) -> AstResult<Option<Self>> {
    tokens.skip_noop();
    if tokens.is_empty() {
      return Ok(None);
    }

    if let Some(function) = Function::try_function(tokens) {}
  }
}
