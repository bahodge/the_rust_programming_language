// fn main() {
//     let x = 5;
//     let y = &x;

//     assert_eq!(5, x);
//     assert_eq!(5, *y); // follow the pointer to the value
// }

// fn main() {
//     let x = 5;
//     let y = Box::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello {}", name);
}

fn main() {
    //     let x = 5;
    //     let y = MyBox::new(x);

    //     assert_eq!(5, x);
    //     assert_eq!(5, *y);
    let ben_box = MyBox::new(String::from("ben_box"));
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);

    hello(&ben_box);

    hello("World");
}
