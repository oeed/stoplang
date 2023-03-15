use std::collections::HashMap;

use crate::ast::{identifier::Identifier, Location};

use super::{variable::Variable, RuntimeError, RuntimeResult};

pub struct Scope<'a> {
  pub variables: HashMap<Identifier, Variable<'a>>,
}

impl<'a> Scope<'a> {
  pub fn new() -> Self {
    Scope {
      variables: HashMap::new(),
    }
  }

  fn get(&self, name: &Identifier) -> Option<&Variable<'a>> {
    self.variables.get(&name)
  }

  fn get_mut(&mut self, name: &Identifier) -> Option<&mut Variable<'a>> {
    self.variables.get_mut(&name)
  }

  pub fn set(&mut self, name: Identifier, variable: Variable<'a>) {
    self.variables.insert(name, variable);
  }

  fn print(&self) {
    for (name, variable) in &self.variables {
      println!("{}", name.0);
    }
  }
}

pub struct ScopeStack<'a>(Vec<Scope<'a>>, Scope<'a>);

impl<'a> ScopeStack<'a> {
  pub fn new() -> Self {
    let mut global_scope = Scope::new();
    std_lib(&mut global_scope);
    ScopeStack(vec![Scope::new()], global_scope)
  }

  pub fn get(&self, name: &Identifier, location: Location) -> RuntimeResult<&Variable<'a>> {
    for scope in self.0.iter().rev() {
      if let Some(var) = scope.get(name) {
        return Ok(var);
      }
    }

    if let Some(var) = self.1.get(name) {
      return Ok(var);
    }

    Err(RuntimeError::UnknownVariable {
      name: name.0.to_string(),
      location,
    })
  }

  pub fn get_mut(&mut self, name: &Identifier, location: Location) -> RuntimeResult<&mut Variable<'a>> {
    for scope in self.0.iter_mut().rev() {
      if let Some(var) = scope.get_mut(name) {
        return Ok(var);
      }
    }

    if let Some(var) = self.1.get_mut(name) {
      return Ok(var);
    }

    Err(RuntimeError::UnknownVariable {
      name: name.0.to_string(),
      location,
    })
  }

  pub fn set(&mut self, name: Identifier, variable: Variable<'a>) {
    self.0.last_mut().unwrap().set(name, variable);
  }

  pub fn push(&mut self) {
    self.0.push(Scope::new());
  }

  pub fn pop(&mut self) {
    self.0.pop();
  }

  pub fn print(&self) {
    for scope in &self.0 {
      println!("[SCOPE START]");
      scope.print();
      println!("[SCOPE END]");
      println!("");
    }
    println!("[GLOBAL SCOPE START]");
    self.1.print();
    println!("[GLOBAL SCOPE END]");
  }
}

fn std_lib(global_scope: &mut Scope) {
  global_scope.set(
    Identifier(String::from("print")),
    Variable::NativeFunction(|args| {
      for arg in args {
        match arg {
          Variable::String(string) => print!("{}", string),
          Variable::Number(number) => print!("{}", number),
          Variable::Bool(true) => print!("true"),
          Variable::Bool(false) => print!("false"),
          Variable::List(list) => {
            print!("[");
            for (i, item) in list.iter().enumerate() {
              if i != 0 {
                print!(", ");
              }
              print!("{}", item);
            }
            print!("]");
          }
          Variable::Nil => print!("nil"),
          Variable::NativeFunction(_) => print!("function"),
          Variable::Function(_) => print!("function"),
          Variable::Map(map) => {
            print!("{{");
            for (i, (key, value)) in map.iter().enumerate() {
              if i != 0 {
                print!(", ");
              }
              print!("\"{}\": {}", key, value);
            }
            print!("}}");
          }
        }
      }
      println!();
      Variable::Nil
    }),
  );
  global_scope.set(
    Identifier(String::from("push")),
    Variable::NativeFunction(|args| {
      assert_eq!(args.len(), 2);
      let mut args = args.into_iter();
      let first = args.next().unwrap();
      let second = args.next().unwrap();

      match first {
        Variable::List(mut list) => {
          list.push(second);
          Variable::List(list)
        }
        _ => panic!("push() can only be called on lists"),
      }
    }),
  );
  global_scope.set(
    Identifier(String::from("pop")),
    Variable::NativeFunction(|args| {
      assert_eq!(args.len(), 1);
      let first = args.into_iter().next().unwrap();

      match first {
        Variable::List(mut list) => {
          let last = list.pop().unwrap();
          Variable::List(list)
        }
        _ => panic!("pop() can only be called on lists"),
      }
    }),
  );
  global_scope.set(
    Identifier(String::from("len")),
    Variable::NativeFunction(|args| {
      assert_eq!(args.len(), 1);
      let first = args.into_iter().next().unwrap();

      match first {
        Variable::String(string) => Variable::Number(string.len() as f64),
        Variable::List(list) => Variable::Number(list.len() as f64),
        _ => panic!("len() can only be called on strings and lists"),
      }
    }),
  );
  global_scope.set(
    Identifier(String::from("input")),
    Variable::NativeFunction(|args| {
      assert_eq!(args.len(), 0);
      let mut input = String::new();
      std::io::stdin().read_line(&mut input).unwrap();
      Variable::String(input)
    }),
  );
  global_scope.set(
    Identifier(String::from("type")),
    Variable::NativeFunction(|args| {
      assert_eq!(args.len(), 1);
      let first = args.into_iter().next().unwrap();

      match first {
        Variable::String(_) => Variable::String("string".to_string()),
        Variable::Number(_) => Variable::String("number".to_string()),
        Variable::Bool(_) => Variable::String("bool".to_string()),
        Variable::List(_) => Variable::String("list".to_string()),
        Variable::Nil => Variable::String("nil".to_string()),
        Variable::NativeFunction(_) => Variable::String("function".to_string()),
        Variable::Function(_) => Variable::String("function".to_string()),
        Variable::Map(_) => Variable::String("map".to_string()),
      }
    }),
  );
  global_scope.set(
    Identifier(String::from("range")),
    Variable::NativeFunction(|args| {
      assert_eq!(args.len(), 2);
      let mut args = args.into_iter();
      let first = args.next().unwrap();
      let second = args.next().unwrap();

      match (first, second) {
        (Variable::Number(first), Variable::Number(second)) => {
          let mut list = Vec::new();
          for i in first as usize..second as usize {
            list.push(Variable::Number(i as f64));
          }
          Variable::List(list)
        }
        _ => panic!("range() can only be called on numbers"),
      }
    }),
  );

  global_scope.set(
    Identifier(String::from("sort")),
    Variable::NativeFunction(|args| {
      assert_eq!(args.len(), 1);
      let first = args.into_iter().next().unwrap();

      match first {
        Variable::List(mut list) => {
          list.sort_by(|a, b| {
            if let (Variable::Number(a), Variable::Number(b)) = (a, b) {
              a.partial_cmp(b).unwrap()
            } else {
              panic!("sort() can only be called on lists of numbers")
            }
          });

          Variable::List(list)
        }
        _ => panic!("sort() can only be called on lists"),
      }
    }),
  );
  global_scope.set(
    Identifier(String::from("number")),
    Variable::NativeFunction(|args| {
      assert_eq!(args.len(), 1);
      let first = args.into_iter().next().unwrap();

      match first {
        Variable::String(string) => Variable::Number(string.trim().parse::<f64>().unwrap()),
        Variable::Number(number) => Variable::Number(number),
        _ => panic!("number() can only be called on strings and numbers"),
      }
    }),
  );
  global_scope.set(
    Identifier(String::from("string")),
    Variable::NativeFunction(|args| {
      assert_eq!(args.len(), 1);
      let first = args.into_iter().next().unwrap();

      match first {
        Variable::String(string) => Variable::String(string),
        Variable::Number(number) => Variable::String(number.to_string()),
        _ => panic!("string() can only be called on strings and numbers"),
      }
    }),
  );
  global_scope.set(
    Identifier(String::from("bool")),
    Variable::NativeFunction(|args| {
      assert_eq!(args.len(), 1);
      let first = args.into_iter().next().unwrap();

      match first {
        Variable::String(string) => Variable::Bool(string.parse().unwrap()),
        Variable::Number(number) => Variable::Bool(number != 0.0),
        _ => panic!("bool() can only be called on strings and numbers"),
      }
    }),
  );
  global_scope.set(
    Identifier(String::from("list")),
    Variable::NativeFunction(|args| {
      let mut list = Vec::new();
      for arg in args {
        list.push(arg);
      }
      Variable::List(list)
    }),
  );
  global_scope.set(
    Identifier(String::from("format")),
    Variable::NativeFunction(|args| {
      assert_eq!(args.len(), 2);
      let mut args = args.into_iter();
      let first = args.next().unwrap();
      let second = args.next().unwrap();

      match (first, second) {
        (Variable::String(string), Variable::List(list)) => {
          let mut string = string;
          for item in list {
            string = string.replace("{}", &format!("{}", item));
          }
          Variable::String(string)
        }
        _ => panic!("format() can only be called on a string and a list"),
      }
    }),
  );
}
