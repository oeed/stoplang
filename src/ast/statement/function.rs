use crate::{
  ast::{identifier::Identifier, AstResult, Location},
  token::{Grammar, Keyword, TokenStream},
};
use derive_more::Display;

use super::Statement;

#[derive(Debug, PartialEq, Clone, Display)]
#[display(fmt = "Function({})", name)]
pub struct Function<'a> {
  pub name: Identifier<'a>,
  pub arguments: Vec<Identifier<'a>>,
  pub block: Vec<Statement<'a>>,
  pub location: Location,
}

impl<'a> Function<'a> {
  pub fn try_function_opt(tokens: &mut TokenStream<'a>) -> AstResult<Option<Self>> {
    if tokens.try_keyword(Keyword::Fn).is_err() {
      return Ok(None);
    }

    let location = tokens.location();
    let name = tokens.try_identifier()?;
    tokens.try_grammar(Grammar::CloseBracket)?;

    let mut arguments = Vec::new();
    loop {
      if tokens.try_grammar(Grammar::OpenBracket).is_ok() {
        // end of arguments
        break;
      }
      arguments.push(tokens.try_identifier()?);

      if tokens.try_grammar(Grammar::Comma).is_err() {
        // no comma, this must also be the end of arguments, expect an open bracket
        tokens.try_grammar(Grammar::OpenBracket)?;
        break;
      }
    }

    let block = Statement::try_block(tokens)?;

    Ok(Some(Function {
      name,
      arguments,
      block,
      location,
    }))
  }
}

// #[cfg(test)]
// mod tests {
//   use crate::ast::expression::Expression;

//   use super::*;

//   #[test]
//   fn function() {
//     let mut tokens = TokenStream::new(
//       "
//       {
//         1
//       } (2arg, 1arg) func_name fn",
//     );
//     assert_eq!(
//       Function::try_function_opt(&mut tokens),
//       Ok(Some(Function {
//         name: Identifier("func_name"),
//         arguments: vec![Identifier("1arg"), Identifier("2arg"),],
//         block: vec![Statement::Expression(Expression::Number(1.))]
//       }))
//     );
//     assert_eq!(Function::try_function_opt(&mut tokens), Ok(None));
//   }
// }
