use crate::{
  ast::{expression::Expression, AstResult, Location},
  token::{Keyword, TokenStream},
};

use super::Statement;

#[derive(Debug, PartialEq, Clone)]
pub struct Conditional<'a> {
  // no else if for now to keep things simple
  pub condition: Expression<'a>,
  pub true_block: Vec<Statement<'a>>,
  pub false_block: Vec<Statement<'a>>,
  pub location: Location,
}

impl<'a> Conditional<'a> {
  pub fn try_conditional_opt(tokens: &mut TokenStream<'a>) -> AstResult<Option<Self>> {
    if tokens.try_keyword(Keyword::If).is_err() {
      return Ok(None);
    }

    let location = tokens.location();
    let condition = Expression::try_expression(tokens)?;
    let true_block = Statement::try_block(tokens)?;
    if tokens.try_keyword(Keyword::Else).is_ok() {
      let false_block = Statement::try_block(tokens)?;
      Ok(Some(Conditional {
        condition,
        true_block,
        false_block,
        location,
      }))
    } else {
      Ok(Some(Conditional {
        condition,
        true_block,
        false_block: Vec::new(),
        location,
      }))
    }
  }

  pub fn print(&self, indent: usize) {
    println!("{}if", " ".repeat(indent));
    self.condition.print(indent + 2);
    println!("{}then", " ".repeat(indent));
    for statement in &self.true_block {
      statement.print(indent + 2);
    }
    if !self.false_block.is_empty() {
      println!("{}else", " ".repeat(indent));
      for statement in &self.false_block {
        statement.print(indent + 2);
      }
    }
  }
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//   use crate::token::Operator;

//   #[test]
//   fn conditional_else() {
//     let mut tokens = TokenStream::new(
//       "{
//         2
//       } else
//       {
//         1
//       } true || false if",
//     );
//     assert_eq!(
//       Conditional::try_conditional_opt(&mut tokens),
//       Ok(Some(Conditional {
//         condition: Expression::Operation {
//           operator: Operator::Or,
//           left: Box::new(Expression::Bool(true)),
//           right: Box::new(Expression::Bool(false))
//         },
//         true_block: vec![Statement::Expression(Expression::Number(2.))],
//         false_block: vec![Statement::Expression(Expression::Number(1.))]
//       }))
//     );
//     assert_eq!(Conditional::try_conditional_opt(&mut tokens), Ok(None));
//   }

//   #[test]
//   fn conditional() {
//     let mut tokens = TokenStream::new(
//       "
//       {
//         1
//       } true || false if",
//     );
//     assert_eq!(
//       Conditional::try_conditional_opt(&mut tokens),
//       Ok(Some(Conditional {
//         condition: Expression::Operation {
//           operator: Operator::Or,
//           left: Box::new(Expression::Bool(true)),
//           right: Box::new(Expression::Bool(false))
//         },
//         true_block: vec![],
//         false_block: vec![Statement::Expression(Expression::Number(1.))]
//       }))
//     );
//     assert_eq!(Conditional::try_conditional_opt(&mut tokens), Ok(None));
//   }

//   #[test]
//   fn conditional_brackets() {
//     let mut tokens = TokenStream::new(
//       "
//       {
//         1
//       }
//       (true) if",
//     );
//     assert_eq!(
//       Conditional::try_conditional_opt(&mut tokens),
//       Ok(Some(Conditional {
//         condition: Expression::Brackets(Box::new(Expression::Bool(true))),
//         true_block: vec![],
//         false_block: vec![Statement::Expression(Expression::Number(1.))]
//       }))
//     );
//     assert_eq!(Conditional::try_conditional_opt(&mut tokens), Ok(None));
//   }
// }
