use crate::{
  interpreter::{RuntimeError, RuntimeResult},
  token::{Grammar, Keyword, Operator, TokenStream},
};

use super::{identifier::Identifier, AstError, AstResult};

#[derive(Debug, PartialEq, Clone)]
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
  Brackets(Box<Expression<'a>>),
}

impl<'a> Expression<'a> {
  pub fn try_expression(tokens: &mut TokenStream<'a>) -> AstResult<Self> {
    // TODO: maybe allow for backets?
    let right = if tokens.try_keyword(Keyword::True).is_ok() {
      Expression::Bool(true)
    } else if tokens.try_keyword(Keyword::False).is_ok() {
      Expression::Bool(false)
    } else if let Some(string) = tokens.try_string_opt()? {
      Expression::String(string)
    } else if let Some(number) = tokens.try_number_opt()? {
      Expression::Number(number)
    } else if tokens.try_grammar(Grammar::CloseBracket).is_ok() {
      let expression = Expression::try_expression(tokens)?;
      tokens.try_grammar(Grammar::OpenBracket)?;
      Expression::Brackets(Box::new(expression))
    } else if let Some(identifier) = tokens.try_identifier_opt()? {
      // see if there are brackets, indicating a function call
      if tokens.try_grammar(Grammar::CloseBracket).is_ok() {
        let mut arguments = Vec::new();
        loop {
          if tokens.try_grammar(Grammar::OpenBracket).is_ok() {
            // end of arguments
            break;
          }
          arguments.push(Expression::try_expression(tokens)?);

          if tokens.try_grammar(Grammar::Comma).is_err() {
            // no comma, this must also be the end of arguments, expect an open bracket
            tokens.try_grammar(Grammar::OpenBracket)?;
            break;
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

  pub fn try_into_identifier(&self) -> RuntimeResult<Identifier<'a>> {
    match self {
      Expression::Identifier(identifier) => Ok(*identifier),
      _ => Err(RuntimeError::InvalidExpression { expected: "identifier" }),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bool_true() {
    let mut tokens = TokenStream::new("true");
    assert_eq!(Expression::try_expression(&mut tokens), Ok(Expression::Bool(true)));
  }

  #[test]
  fn bool_false() {
    let mut tokens = TokenStream::new("false");
    assert_eq!(Expression::try_expression(&mut tokens), Ok(Expression::Bool(false)));
  }

  #[test]
  fn brackets() {
    let mut tokens = TokenStream::new("(false)");
    assert_eq!(
      Expression::try_expression(&mut tokens),
      Ok(Expression::Brackets(Box::new(Expression::Bool(false))))
    );
  }

  #[test]
  fn string() {
    let mut tokens = TokenStream::new("\"hello there\"");
    assert_eq!(
      Expression::try_expression(&mut tokens),
      Ok(Expression::String("hello there"))
    );
  }

  #[test]
  fn number() {
    let mut tokens = TokenStream::new("42.24");
    assert_eq!(Expression::try_expression(&mut tokens), Ok(Expression::Number(42.24)));
  }

  #[test]
  fn identifier() {
    let mut tokens = TokenStream::new("123 my_ident");
    assert_eq!(
      Expression::try_expression(&mut tokens),
      Ok(Expression::Identifier(Identifier("my_ident")))
    );
  }

  #[test]
  fn call() {
    let mut tokens = TokenStream::new("(123, \"hello\")my_func");
    assert_eq!(
      Expression::try_expression(&mut tokens),
      Ok(Expression::Call {
        function: Identifier("my_func"),
        arguments: vec![Expression::String("hello"), Expression::Number(123.)]
      })
    );
  }

  #[test]
  fn operator() {
    let mut tokens = TokenStream::new("\"hello\" <= 99");
    assert_eq!(
      Expression::try_expression(&mut tokens),
      Ok(Expression::Operation {
        operator: Operator::Lte,
        left: Box::new(Expression::String("hello")),
        right: Box::new(Expression::Number(99.)),
      })
    );
  }

  #[test]
  fn operator_missing_left() {
    let mut tokens = TokenStream::new(" + 99");
    assert!(Expression::try_expression(&mut tokens).is_err(),);
  }
}
