mod math; // import math

fn main() {
  let x = 100;
  let mut y = 100;

  let mut y_greater_than_x = math::compare(&x, &y, |a, b| a < b);
  println!("y_greater_than_x {}", y_greater_than_x);
  y = math::add_one(&y);
  println!("y {}", y);
  y = math::nested::add_two(&y);
  println!("y {}", y);
  y = math::add_five(&y);
  println!("y {}", y);

  y_greater_than_x = math::compare(&x, &y, |a, b| a < b);
  println!("y_greater_than_x {}", y_greater_than_x);
}

pub fn hello() -> String {
  String::from("Hello world!")
}

pub fn goodbye() -> String {
  String::from("Goodbye world!")
}

#[cfg(test)]
mod tests {

  use crate::{goodbye, hello};

  #[test]
  fn hello_returns_hello() {
    assert_eq!(hello(), String::from("Hello world!"));
  }
  #[test]
  fn goodbye_returns_goodbye() {
    assert_eq!(goodbye(), String::from("Goodbye world!"));
  }
}
