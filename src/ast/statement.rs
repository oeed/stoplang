use super::{expression::Expression, identifier::Identifier};

pub enum Statement<'a> {
  Assignment {
    variable: Identifier<'a>,
    value: Expression<'a>,
  },
  Conditional {
    // no else if for now to keep things simple
    condition: Expression<'a>,
    true_block: Vec<Statement<'a>>,
    false_block: Vec<Statement<'a>>,
  },
}
