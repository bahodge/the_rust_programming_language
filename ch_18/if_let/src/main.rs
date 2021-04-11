// fn print_coordinates(&(x, y): &(i32, i32)) {
//     println!("Current location: ({}, {})", x, y);
// }
fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }

    // let x = 4;
    // let y = true;

    // match x {
    //     4 | 5 | 6 if y => println!("yes"),
    //     _ => println!("no"),
    // }

    // let point = (3, 5);
    // print_coordinates(&point);
    // println!("Current location: {:?}", point);

    // let v = vec!['a', 'b', 'c'];

    // for (index, value) in v.iter().enumerate() {
    //     println!("{} is at index {}", value, index);
    // }
    // let mut stack = Vec::new();
    // stack.push(1);
    // stack.push(2);
    // stack.push(3);

    // while let Some(top) = stack.pop() {
    //     println!("{}", top)
    // }

    // let favorite_color: Option<&str> = None;
    // let is_tuesday = false;
    // let age: Result<u8, _> = "34".parse();

    // if let Some(color) = favorite_color {
    //     println!("Using your favorite color, {}, as the background", color);
    // } else if is_tuesday {
    //     println!("Tuesday is green day!");
    // } else if let Ok(age) = age {
    //     if age > 30 {
    //         println!("Using purple as the background color");
    //     } else {
    //         println!("Using orange as the background color");
    //     }
    // } else {
    //     println!("Using blue as the background color");
    // }
}
