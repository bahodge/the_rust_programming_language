use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();

  let var = 1;

  map.insert(var, 2);

  println!("Map: {}", map.get(&var).expect("asdfasdf"));
}
