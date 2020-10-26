fn main() {

  // USING AN ENUM AS THE DATATYPE
  // #[derive(Debug)]
  // enum SpreadsheetCell {
  //   Int(i32),
  //   Float(f64),
  //   Text(String),
  // }

  // let row: Vec<SpreadsheetCell> = vec![
  //   SpreadsheetCell::Int(3),
  //   SpreadsheetCell::Float(1.3),
  //   SpreadsheetCell::Text(String::from("hello")),
  // ];

  // for cell in &row {
  //   println!("Cell: {:?}", cell)
  // }

  // // ITERATING OVER MUTABLE
  // let mut v = vec![1, 2, 3];

  // for i in &mut v {
  // // (*) is the dereference operator
  //   *i += 50;
  //   println!("Hello, {}", i)
  // }

  // // ITERATING
  // let v = vec![1, 2, 3];

  // for i in &v {
  //   println!("Hello, {}", i);
  // }

  // // PUSHING PANIC
  // let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
  // let first = &v[0];

  // v.push(6); // FAILS TO COMPILE

  // println!("The firset element is {}", first);

  // // ARRAY OUT OF BOUNDS
  // let v = vec![1, 2, 3, 4, 5];

  // let does_not_exist = &v[100]; // PANIC
  // let does_not_exist = v.get(100); // NO PANIC

  // // Reading
  // let v = vec![1, 2, 3];

  // // ARRAY INDEX
  // let third: &i32 = &v[2];

  // println!("The third element is {}", third);
  // println!("the length is {}", v.len());

  // // GET
  // match v.get(2) {
  //   Some(third) => println!("The third element is {}", third),
  //   None => println!("There is no third element"),
  // }

  // Push
  // let mut v: Vec<i32> = Vec::new();

  // v.push(5);
  // v.push(6);
  // v.push(7);
  // v.push(8);

  // for int in v {
  //   println!("int: {}", int);
  // }

  // MACRO
  // let v: Vec<i32> = vec![1, 2, 3];

  // NATIVE
  // let v: Vec<i32> = Vec::new();
}
