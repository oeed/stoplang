use stoplang::token::TokenStream;

fn main() {
  let file: String = std::fs::read_to_string("examples/example.stop")
    .unwrap()
    .parse()
    .unwrap();
  let mut token_stream = TokenStream::new(&file);
}
