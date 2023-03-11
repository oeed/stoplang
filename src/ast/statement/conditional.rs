use crate::{
  ast::{expression::Expression, identifier::Identifier, AstResult},
  token::{Keyword, Operator, TokenStream},
};

use super::Statement;

pub struct Conditional<'a> {
  // no else if for now to keep things simple
  condition: Expression<'a>,
  true_block: Vec<Statement<'a>>,
  false_block: Vec<Statement<'a>>,
}

impl<'a> Conditional<'a> {
  pub fn try_conditional_opt(tokens: &mut TokenStream<'a>) -> AstResult<Option<Self>> {
    if tokens.try_keyword(Keyword::If).is_err() {
      return Ok(None);
    }

    let condition = Expression::try_expression(tokens);
  }
}
