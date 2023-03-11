use self::statement::Statement;

pub mod expression;
pub mod identifier;
pub mod statement;

pub struct Ast<'a> {
  statements: Vec<Statement<'a>>,
}

impl<'a> Ast<'a> {
  // pub fn new(tokens: &'a str) -> Self {}
}
