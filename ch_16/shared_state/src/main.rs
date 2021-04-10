use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let initial_value = 0;
    let counter = Arc::new(Mutex::new(initial_value));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("initial_value: {}", initial_value);
    println!("Result: {}", *counter.lock().unwrap());
}

// // Reference Counter cannot be sent safely between threads;
// use std::rc::Rc;
// use std::sync::Mutex;
// use std::thread;

// fn main() {
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

// References
// use std::sync::Mutex;
// use std::thread;

// fn main() {
//     let counter: Mutex<i32> = Mutex::new(0);
//     let mut handles = vec![];

//     for _ in 0..10 {
//         // counter was moved on the previous iteration
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     println!("Result: {}", *counter.lock().unwrap());
// }

// // Single thread mutex
// use std::sync::Mutex;

// fn main() {
//     let m = Mutex::new(10);

//     {
//         let mut num = m.lock().unwrap();

//         *num = 6;
//     }

//     println!("m = {:?}", m); // 6
// }
