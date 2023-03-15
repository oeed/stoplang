use derive_more::Display;

use super::Statement;
use crate::{
  ast::{expression::Expression, AstResult, Location},
  token::{Keyword, TokenStream},
};

#[derive(Debug, PartialEq, Clone, Display)]
#[display(fmt = "While")]
pub struct While<'a> {
  pub condition: Expression<'a>,
  pub block: Vec<Statement<'a>>,
  pub location: Location,
}

impl<'a> While<'a> {
  pub fn try_while_opt(tokens: &mut TokenStream<'a>) -> AstResult<Option<Self>> {
    if tokens.try_keyword(Keyword::While).is_err() {
      return Ok(None);
    }

    let location = tokens.location();
    let condition = Expression::try_expression(tokens)?;
    let block = Statement::try_block(tokens)?;

    Ok(Some(While {
      condition,
      block,
      location,
    }))
  }
}
