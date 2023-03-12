use stoplang::{ast::Ast, interpreter::interpret, token::TokenStream};

fn main() {
  let file: String = std::fs::read_to_string("examples/hello.stop").unwrap().parse().unwrap();
  let mut tokens = TokenStream::new(&file);
  let ast = Ast::new(&mut tokens).unwrap();
  interpret(ast).unwrap();
}
