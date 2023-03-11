fn main() {
  let file: String = std::fs::read_to_string("examples/example.stop")
    .unwrap()
    .parse()
    .unwrap();
  Ast::new()
}
