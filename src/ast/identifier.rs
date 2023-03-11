pub struct Identifier<'a>(pub &'a str);

impl<'a> Identifier<'a> {
  pub fn is_valid_first_char(char: char) -> bool {
    char.is_ascii_alphabetic() || char == '_'
  }

  pub fn is_valid_char(char: char) -> bool {
    char.is_ascii_alphanumeric() || char == '_'
  }
}
