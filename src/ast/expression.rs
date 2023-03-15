use std::collections::HashMap;

use super::{identifier::Identifier, AstError, AstResult, Location};
use crate::{
  interpreter::{RuntimeError, RuntimeResult},
  token::{Grammar, Keyword, Operator, TokenStream},
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression<'a> {
  Bool(bool, Location),
  String(&'a str, Location),
  Number(f64, Location),
  List(Vec<Expression<'a>>, Location),
  Map(HashMap<String, Expression<'a>>, Location),
  Index {
    indexed: Identifier<'a>,
    indices: Vec<Expression<'a>>,
    location: Location,
  },
  Operation {
    operator: Operator,
    left: Box<Expression<'a>>,
    right: Box<Expression<'a>>,
    location: Location,
  },
  Call {
    function: Identifier<'a>,
    arguments: Vec<Expression<'a>>,
    location: Location,
  },
  Identifier(Identifier<'a>, Location),
  Brackets(Box<Expression<'a>>, Location),
}

impl<'a> Expression<'a> {
  pub fn try_expression(tokens: &mut TokenStream<'a>) -> AstResult<Self> {
    // TODO: maybe allow for backets?
    let right = if tokens.try_keyword(Keyword::True).is_ok() {
      Expression::Bool(true, tokens.location())
    } else if tokens.try_keyword(Keyword::False).is_ok() {
      Expression::Bool(false, tokens.location())
    } else if let Some(string) = tokens.try_string_opt()? {
      Expression::String(string, tokens.location())
    } else if let Some(number) = tokens.try_number_opt()? {
      Expression::Number(number, tokens.location())
    } else if tokens.try_grammar(Grammar::CloseBracket).is_ok() {
      let expression = Expression::try_expression(tokens)?;
      tokens.try_grammar(Grammar::OpenBracket)?;
      Expression::Brackets(Box::new(expression), tokens.location())
    } else if tokens.try_grammar(Grammar::CloseCurly).is_ok() {
      let mut map = HashMap::new();
      loop {
        if tokens.try_grammar(Grammar::OpenCurly).is_ok() {
          break;
        }
        let key = if let Some(string) = tokens.try_string_opt()? {
          string.to_owned()
        } else {
          tokens.try_identifier()?.0.to_owned()
        };
        if map.contains_key(&key) {
          return Err(AstError::DuplicateKey(tokens.location()));
        }
        tokens.try_grammar(Grammar::Colon)?;

        let value = Expression::try_expression(tokens)?;
        map.insert(key, value);
        if tokens.try_grammar(Grammar::Comma).is_err() {
          tokens.try_grammar(Grammar::OpenCurly)?;
          break;
        }
      }
      Expression::Map(map, tokens.location())
    } else if tokens.try_grammar(Grammar::SquareClose).is_ok() {
      let mut expressions = Vec::new();
      loop {
        if tokens.try_grammar(Grammar::SquareOpen).is_ok() {
          break;
        }
        expressions.push(Expression::try_expression(tokens)?);
        if tokens.try_grammar(Grammar::Comma).is_err() {
          tokens.try_grammar(Grammar::SquareOpen)?;
          break;
        }
      }
      Expression::List(expressions, tokens.location())
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
          location: tokens.location(),
        }
      } else if tokens.try_grammar(Grammar::SquareClose).is_ok() {
        // support multiple indexes ie a[0][1]
        let mut indices = Vec::new();
        indices.push(Expression::try_expression(tokens)?);

        loop {
          tokens.try_grammar(Grammar::SquareOpen)?;

          if tokens.try_grammar(Grammar::SquareClose).is_err() {
            break;
          }
          indices.push(Expression::try_expression(tokens)?);
        }
        Expression::Index {
          indexed: identifier,
          indices,
          location: tokens.location(),
        }
      } else {
        Expression::Identifier(identifier, tokens.location())
      }
    } else {
      return Err(AstError::MissingExpression(tokens.location()));
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
        location: tokens.location(),
      })
    } else {
      Ok(right)
    }
  }

  pub fn location(&self) -> Location {
    match self {
      Expression::Bool(_, location)
      | Expression::Identifier(_, location)
      | Expression::Brackets(_, location)
      | Expression::String(_, location)
      | Expression::Number(_, location)
      | Expression::Operation { location, .. }
      | Expression::Call { location, .. } => *location,
      Expression::List(_, location) => *location,
      Expression::Index { location, .. } => *location,
      Expression::Map(_, location) => *location,
    }
  }

  pub fn try_into_identifier(&self) -> RuntimeResult<Identifier<'a>> {
    match self {
      Expression::Identifier(identifier, _) => Ok(*identifier),
      _ => Err(RuntimeError::InvalidExpression {
        expected: "identifier",
        location: self.location(),
      }),
    }
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn bool_true() {
//     let mut tokens = TokenStream::new("true");
//     assert_eq!(
//       Expression::try_expression(&mut tokens),
//       Ok(Expression::Bool(true, Location::new(None)))
//     );
//   }

//   #[test]
//   fn bool_false() {
//     let mut tokens = TokenStream::new("false");
//     assert_eq!(
//       Expression::try_expression(&mut tokens),
//       Ok(Expression::Bool(false, Location::new(None)))
//     );
//   }

//   #[test]
//   fn brackets() {
//     let mut tokens = TokenStream::new("(false)");
//     assert_eq!(
//       Expression::try_expression(&mut tokens),
//       Ok(Expression::Brackets(
//         Box::new(Expression::Bool(false, Location::new(None))),
//         Location::new(None)
//       ))
//     );
//   }

//   #[test]
//   fn string() {
//     let mut tokens = TokenStream::new("\"hello there\"");
//     assert_eq!(
//       Expression::try_expression(&mut tokens),
//       Ok(Expression::String("hello there"))
//     );
//   }

//   #[test]
//   fn number() {
//     let mut tokens = TokenStream::new("42.24");
//     assert_eq!(Expression::try_expression(&mut tokens), Ok(Expression::Number(42.24)));
//   }

//   #[test]
//   fn identifier() {
//     let mut tokens = TokenStream::new("123 my_ident");
//     assert_eq!(
//       Expression::try_expression(&mut tokens),
//       Ok(Expression::Identifier(Identifier("my_ident")))
//     );
//   }

//   #[test]
//   fn call() {
//     let mut tokens = TokenStream::new("(123, \"hello\")my_func");
//     assert_eq!(
//       Expression::try_expression(&mut tokens),
//       Ok(Expression::Call {
//         function: Identifier("my_func"),
//         arguments: vec![Expression::String("hello"), Expression::Number(123.)]
//       })
//     );
//   }

//   #[test]
//   fn operator() {
//     let mut tokens = TokenStream::new("\"hello\" <= 99");
//     assert_eq!(
//       Expression::try_expression(&mut tokens),
//       Ok(Expression::Operation {
//         operator: Operator::Lte,
//         left: Box::new(Expression::String("hello")),
//         right: Box::new(Expression::Number(99.)),
//       })
//     );
//   }

//   #[test]
//   fn operator_missing_left() {
//     let mut tokens = TokenStream::new(" + 99");
//     assert!(Expression::try_expression(&mut tokens).is_err(),);
//   }
// }
