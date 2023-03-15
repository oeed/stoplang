use crate::{
  interpreter::{RuntimeError, RuntimeResult},
  token::{Grammar, Keyword, Operator, TokenStream},
};

use super::{
  identifier::{self, Identifier},
  AstError, AstResult, Location,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression<'a> {
  Bool(bool, Location),
  String(&'a str, Location), // TODO: given we want to reverse, maybe use owned?
  Number(f64, Location),
  List(Vec<Expression<'a>>, Location),
  Map(Vec<(Identifier, Expression<'a>)>, Location),
  Index(Identifier, Vec<Expression<'a>>, Location),
  Operation {
    operator: Operator,
    left: Box<Expression<'a>>,
    right: Box<Expression<'a>>,
    location: Location,
  },
  Call {
    function: Identifier,
    arguments: Vec<Expression<'a>>,
    location: Location,
  },
  Identifier(Identifier, Location),
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
      let mut expressions = Vec::new();
      loop {
        if tokens.try_grammar(Grammar::OpenCurly).is_ok() {
          break;
        }
        let identifier = tokens.try_identifier()?;
        tokens.try_grammar(Grammar::Colon)?;
        let expression = Expression::try_expression(tokens)?;
        expressions.push((identifier, expression));
        if tokens.try_grammar(Grammar::Comma).is_err() {
          tokens.try_grammar(Grammar::OpenCurly)?;
          break;
        }
      }
      Expression::Map(expressions, tokens.location())
    } else if tokens.try_grammar(Grammar::ListClose).is_ok() {
      let mut expressions = Vec::new();
      loop {
        if tokens.try_grammar(Grammar::ListOpen).is_ok() {
          break;
        }
        expressions.push(Expression::try_expression(tokens)?);
        if tokens.try_grammar(Grammar::Comma).is_err() {
          tokens.try_grammar(Grammar::ListOpen)?;
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
      } else if tokens.try_grammar(Grammar::ListClose).is_ok() {
        // support multiple indexes ie a[0][1]
        let mut expressions = Vec::new();
        expressions.push(Expression::try_expression(tokens)?);

        loop {
          tokens.try_grammar(Grammar::ListOpen)?;

          if tokens.try_grammar(Grammar::ListClose).is_err() {
            break;
          }
          expressions.push(Expression::try_expression(tokens)?);
        }
        Expression::Index(identifier, expressions, tokens.location())
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
      Expression::Index(_, _, location) => *location,
      Expression::Map(_, location) => *location,
    }
  }

  pub fn try_into_identifier(&self) -> RuntimeResult<Identifier> {
    match self {
      Expression::Identifier(identifier, _) => Ok(identifier.clone()),
      _ => Err(RuntimeError::InvalidExpression {
        expected: "identifier",
        location: self.location(),
      }),
    }
  }

  pub fn print(&self, indent: usize) {
    let indent = " ".repeat(indent);
    match self {
      Expression::Bool(value, _) => println!("{}{}", indent, value),
      Expression::String(value, _) => println!("{}{}", indent, value),
      Expression::Number(value, _) => println!("{}{}", indent, value),
      Expression::List(expressions, _) => {
        println!("{}[", indent);
        for expression in expressions {
          expression.print(indent.len() + 2);
        }
        println!("{}]", indent);
      }
      Expression::Operation {
        operator, left, right, ..
      } => {
        println!("{}{}", indent, operator);
        left.print(indent.len() + 2);
        right.print(indent.len() + 2);
      }
      Expression::Call {
        function, arguments, ..
      } => {
        println!("{}{}", indent, function);
        for argument in arguments {
          argument.print(indent.len() + 2);
        }
      }
      Expression::Identifier(identifier, _) => println!("{}{}", indent, identifier),
      Expression::Brackets(expression, _) => {
        println!("{}{}", indent, "()");
        expression.print(indent.len() + 2);
      }
      Expression::Index(identifier, index, _) => {
        println!("{}{}<", indent, identifier);
        for expression in index {
          expression.print(indent.len() + 2);
        }
        println!("{}>", indent);
      }
      Expression::Map(values, _) => {
        println!("{}{}", indent, "{");
        for (key, value) in values {
          println!("{}\"{}\":", indent, key);
          value.print(indent.len() + 2);
        }
        println!("{}{}", indent, "}");
      }
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
