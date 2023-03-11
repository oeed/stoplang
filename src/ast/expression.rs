pub enum Expression<'a> {
  Bool(bool),
  String(&'a str), // TODO: given we want to reverse, maybe use owned?
  Number(f64),
}
