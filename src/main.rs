use std::env;

use derive_more::From;
use stoplang::{
  ast::{Ast, AstError},
  interpreter::{interpret, RuntimeError},
  token::TokenStream,
};

#[derive(From)]
enum LocatedError {
  Ast(AstError),
  Runtime(RuntimeError),
}

fn run(code: &str) -> Result<(), LocatedError> {
  let mut tokens = TokenStream::new(code);
  let ast = Ast::new(&mut tokens)?;
  interpret(ast)?;
  Ok(())
}

fn main() {
  let mut args: Vec<String> = env::args().collect();
  let path = args.pop().expect("missing path");
  let file: String = std::fs::read_to_string(path).unwrap().parse().unwrap();
  match run(&file) {
    Err(LocatedError::Ast(err)) => println!("syntax error at {}: {}", err.location().description(&file), err),
    Err(LocatedError::Runtime(err)) => println!("runtime error at {}: {}", err.location().description(&file), err),
    Ok(_) => (),
  }
}
