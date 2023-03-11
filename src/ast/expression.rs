pub enum Expression<'a> {
  Bool(bool),
  String(&'static str), // TODO: given we want to reverse, maybe use owned?
  Number(f64),
}
