use derive_more::Display;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Display)]
pub struct Identifier(pub String);

impl Identifier {
  pub fn is_valid_first_char(char: char) -> bool {
    char.is_ascii_alphabetic() || char == '_'
  }

  pub fn is_valid_char(char: char) -> bool {
    char.is_ascii_alphanumeric() || char == '_'
  }
}
