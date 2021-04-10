// Cannot guarantee the value will be alive by the time the thread runs clojure
// use std::thread;

// fn main() {
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });

//     // v may not exist when the thread is spawned
//     drop(v); // oh no!

//     handle.join().unwrap();
// }

// // TAKE OWNERSHIP OF V with `move`
// use std::thread;

// fn main() {
//     let v = vec![1, 2, 3];

//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });

//     handle.join().unwrap();
// }

// Using handle to join spawned threads to main thread
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);

//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     // Join to the main thread
//     handle.join().unwrap();

//     for i in 1..5 {
//         println!("hi number {} from the main thread", i);

//         thread::sleep(Duration::from_millis(1));
//     }
// }

// EXITS WHEN MAIN THREAD EXITS
// use std::thread;
// use std::time::Duration;

// fn main() {
//     thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);

//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("hi number {} from the main thread", i);

//         thread::sleep(Duration::from_millis(1));
//     }
// }
