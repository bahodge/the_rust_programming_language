pub fn add_two(x: &i32) -> i32 {
  x + 2
}

#[cfg(test)]
mod tests {
  use super::add_two;
  #[test]
  fn add_two_adds_two() {
    assert_eq!(add_two(&2), 4);
  }
  mod sub_tests {

    use super::add_two;
    #[test]
    fn add_two_doesnt_add_three() {
      assert_ne!(add_two(&2), 3);
    }
  }
}
