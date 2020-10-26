fn main() {

    // println!("value Penny: {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    // let some_u8_value = 0u8;
    // let val = (32, 24, "cat");

    // match val {
    //   (_, _, "cat") => println!("cat!"),
    //   _ => ()
    // }

    // match some_u8_value {
    //     1 => println!("one"),
    //     3 => println!("three"),
    //     5 => println!("five"),
    //     7 => println!("seven"),
    //     _ => (),
    // }

}


// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1)
//     }
// }

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState)
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }