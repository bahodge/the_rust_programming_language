// fn main() {
//     let s = String::from("Hello");

//     takes_ownership(s);

//     //  // Should blow up
//     //  println!("s: {}", s);

//     let x = 5;

//     makes_copy(x);
// }

// fn takes_ownership(s: String) -> String {
//     println!("s: {}", s);
//     s
// }

// fn makes_copy(x: i32) {
//     println!("x: {}", x);
// }

// fn main() {
//     println!("Hello, world!");

//     let s1 = gives_ownership();

//     let s2 = String::from("her derr");

//     let s3 = takes_and_gives_back(s2);
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("Helo");

//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }

fn main() {
    // x is owned
    let x = vec![1, 2, 3];
    // push a bunch takes ownership
    // x is now reassigned to the returned push_a_bunch
    let x = push_a_bunch(x);

    // x is now available again
    println!("pushed a bunch {:?}", x)
}

fn push_a_bunch(mut v: Vec<u32>) -> Vec<u32> {
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v.push(1);
    v
}
