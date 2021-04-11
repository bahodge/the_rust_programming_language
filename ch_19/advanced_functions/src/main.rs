// fn add_one(x: i32) -> i32 {
//     x + 1
// }

// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }

// fn main() {
//     let answer = do_twice(add_one, 5);

//     println!("The answer is: {}", answer);
// }

// fn main() {
//     #[derive(Debug)]
//     enum Status {
//         Value(u8),
//         Stop,
//     }
//     let list_of_statuses: Vec<Status> = (0..255).map(Status::Value).collect();
//     println!("{:?}", list_of_statuses);
// }
fn main() {
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
