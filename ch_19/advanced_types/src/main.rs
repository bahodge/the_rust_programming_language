fn main() {
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    // fn takes_long_type(f: Thunk) {
    //     // --snip--
    // }

    fn returns_long_type() -> Thunk {
        Box::new(|| println!("Hello world"))
        // --snip--
    }

    fn practice(x: i32) -> Thunk {
        let added = 1 + x;

        Box::new(move || println!("p = {}", added))
    }

    let z = 56;
    // type Kilometers = i32;
    // let x: i32 = 5;
    // let y: Kilometers = 5;

    // println!("x + y = {}", x + y);
    // println!("Hello, world!");

    practice(z)();

    println!("z = {}", z);

    // returns_long_type()();
}
