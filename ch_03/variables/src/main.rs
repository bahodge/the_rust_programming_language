fn main() {
    // CONTROL FLOW
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for elem in a.iter() {
        println!("elem: {}", elem);
    }

    for x in (1..4).rev() {
        println!("liftoff in: {}", x);
    }

    println!("Liftoff");

    let mut number: i32 = 3;
    while number > 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("number: {}", number);

    let mut counter = 0;

    let result = loop {
        if counter == 10 {
            break counter * 2;
        }
        println!("counter: {}", counter);
        counter += 1;
    };

    println!("result: {}", result);

    let number = 3;

    if number < 5 {
        println!("true!");
    } else {
        println!("false");
    }

    if number != 0 {
        println!("not zero");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // FUNCTIONS
    fn five() -> i32 {
        5
    }
    let x = five();

    println!("The value of x is: {}", x);

    // ARRAYS
    let arr: [u32; 3] = [1, 2, 3];

    println!("arr: {}", arr[0]);

    // PANICS!!!
    // let a = [1, 2, 3, 4, 5];
    // let index = 10;

    // let element = a[index];

    // println!("The value of element is: {}", element);

    // TUPLES
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // pattern matching / Destructuring
    let (x, y, z) = tup;

    println!("x: {}", x);
    println!("z: {}", z);
    println!("y: {}", y);

    println!("first: {}", tup.0);
    println!("second: {}", tup.1);
    println!("third: {}", tup.2);

    // Characters
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c: {}", c);
    println!("z: {}", z);
    println!("heart_eyed_cat: {}", heart_eyed_cat);

    // BOOLEANS
    let t: bool = true;
    let f: bool = false;

    println!("t: {}", t);
    println!("f: {}", f);

    // Operations
    let sum: u8 = 5 + 10;

    let difference: f32 = 95.5 - 4.3;

    let product: u32 = 4 * 30;

    let quotient: f32 = 56.7 / 32.2;

    let remainder: u8 = 43 % 5;

    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("remainder: {}", remainder);

    // FLOATS
    let x = 2.0;
    let x: f32 = 2.0;

    println!("the value of x is: {}", x);

    // INTEGER OVERFLOW
    let guess: u32 = 256;
    // let guess: u8 = 256; // out of range integer overflow
    println!("the value of guess is: {}", guess);

    // CONSTANTS
    const MAX_POINTS: u32 = 100_000;

    // MUTABLE VARIABLES
    let mut x = 5;
    println!("the value of x is: {}", x);
    x = 6;
    println!("the value of x is: {}", x);

    println!("the max points allowed: {}", MAX_POINTS);

    // SHADOWING
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    // More shadowing
    let spaces = "      ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);
}
