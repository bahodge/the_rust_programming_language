mod internal; // private
pub mod nested; // if you need to include other modules

pub fn add_one(x: &i32) -> i32 {
  x + 1
}

pub fn add_five(x: &i32) -> i32 {
  let plus_three = internal::add_three(x);
  nested::add_two(&plus_three)
}

pub fn compare(x: &i32, y: &i32, f: fn(&i32, &i32) -> bool) -> bool {
  f(x, y)
}

#[cfg(test)]
mod tests {
  use crate::math::{add_one, compare};

  #[test]
  fn add_one_adds_one() {
    let number = 1;
    assert_eq!(add_one(&number), 2);
  }
  #[test]
  fn compare_works() {
    let x = 1;
    let y = 2;
    assert_eq!(compare(&x, &y, |a, b| { a > b }), false);
    assert_eq!(compare(&x, &y, |a, b| { a < b }), true);
  }
}
