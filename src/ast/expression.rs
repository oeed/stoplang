use crate::token::{Grammar, Keyword, Operator, TokenStream};

use super::{identifier::Identifier, AstError, AstResult};

pub enum Expression<'a> {
  Bool(bool),
  String(&'a str), // TODO: given we want to reverse, maybe use owned?
  Number(f64),
  Operation {
    operator: Operator,
    left: Box<Expression<'a>>,
    right: Box<Expression<'a>>,
  },
  Call {
    function: Identifier<'a>,
    arguments: Vec<Expression<'a>>,
  },
  Identifier(Identifier<'a>),
}

impl<'a> Expression<'a> {
  pub fn try_expression(tokens: &mut TokenStream<'a>) -> AstResult<Self> {
    let right = if tokens.try_keyword(Keyword::True).is_ok() {
      Expression::Bool(true)
    } else if tokens.try_keyword(Keyword::False).is_ok() {
      Expression::Bool(false)
    } else if let Some(string) = tokens.try_string_opt()? {
      Expression::String(string)
    } else if let Some(number) = tokens.try_number_opt()? {
      Expression::Number(number)
    } else if let Some(identifier) = tokens.try_identifier_opt()? {
      // see if there are brackets, indicating a function call
      if tokens.try_delimiter(Grammar::CloseBracket).is_ok() {
        let mut arguments = Vec::new();
        loop {
          if tokens.try_delimiter(Grammar::OpenBracket).is_ok() {
            // end of arguments
            break;
          }
          arguments.push(Expression::try_expression(tokens)?);

          if tokens.try_delimiter(Grammar::Comma).is_err() {
            // no comma, this must also be the end of arguments, expect an open bracket
            tokens.try_delimiter(Grammar::OpenBracket)?;
          }
        }
        Expression::Call {
          function: identifier,
          arguments,
        }
      } else {
        Expression::Identifier(identifier)
      }
    } else {
      return Err(AstError::MissingExpression);
    };

    // see if there's an operator
    if let Some(operator) = Operator::operators()
      .iter()
      .find(|op| tokens.try_operator(**op).is_ok())
    {
      let left = Expression::try_expression(tokens)?;
      Ok(Expression::Operation {
        operator: *operator,
        left: Box::new(left),
        right: Box::new(right),
      })
    } else {
      Ok(right)
    }
  }
}
