// #[derive(Debug)]
// struct Person {
//   name: String,
// }

// #[derive(Debug)]
// struct Animal {
//   species: String,
// }

// impl Animal {
//   fn new(species: String) -> Animal {
//     Animal { species }
//   }
// }

// impl Speaker for Animal {
//   fn speak(&self) -> String {
//     format!("I am a {}", self.species)
//   }
// }

// impl Person {
//   fn new(name: String) -> Person {
//     Person { name }
//   }
// }

// impl Speaker for Person {}

// trait Speaker {
//   fn speak(&self) -> String {
//     String::from("Hello!")
//   }
// }

// fn greet(greeter: &impl Speaker) -> String {
//   greeter.speak()
// }

// fn main() {
//   let animal = Animal::new(String::from("dog"));
//   let person = Person::new(String::from("Ben"));

//   println!("animal {:?}", animal);
//   println!("person {:?}", person);
//   println!("animal.speak() {:?}", animal.speak());
//   println!("person.speak() {:?}", person.speak());
//   println!("greet(person) {:?}", greet(&person));
// }

/// LIFETIMES
// fn main() {
//   {
//     let r;

//     {
//       let x = 5;
//       r = &x;
//     }
//     println!("r: {}", r);
//   }
// }
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
  T: Display,
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn main() {
  let string1 = String::from("long string is long");

  {
    let string2 = String::from("xyz");
    let result =
      longest_with_an_announcement(string1.as_str(), string2.as_str(), String::from("Ben!"));
    println!("The longest string is {}", result);
  }
}
