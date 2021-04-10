// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let tx1 = tx.clone();
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];

//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got: {}", received);
//     }
// }

// Sending multiple messages from the first transmitter
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let values = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];

//         for string_value in values {
//             tx.send(string_value).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for received in rx {
//         println!("Got: {}", received);
//     }
// }

// use std::sync::mpsc;
// use std::thread;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//         println!("val is {}", val); // val ownership has already be revoked and sent to another thread
//     });

//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
// }

// Spawn 2 threads with a loop that waits for the first thread to transmit
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     let handle = thread::spawn(move || {
//         let val = String::from("hi");
//         thread::sleep(Duration::from_millis(500));
//         match tx.send(val) {
//             Ok(v) => v,
//             Err(e) => eprint!("error {}", e),
//         };
//     });

//     thread::spawn(move || loop {
//         match rx.try_recv() {
//             Ok(v) => {
//                 println!("msg received {} \n", v);
//                 break ();
//             }
//             Err(_) => (),
//         };
//     });

//     handle.join().unwrap()
// }
