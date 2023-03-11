use stoplang::{ast::Ast, token::TokenStream};

fn main() {
  let file: String = std::fs::read_to_string("examples/fib.stop").unwrap().parse().unwrap();
  let mut tokens = TokenStream::new(&file);
  let ast = Ast::new(&mut tokens).unwrap();
  println!("AST: {:#?}", ast)
}
