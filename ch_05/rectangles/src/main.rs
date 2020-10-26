fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("area of rectangle is: {}", area1(width1, height1));

    let rect1 = (30, 50);

    println!("area of rectangle is: {}", area2(rect1));

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("area of rectangle is: {}", area3(&rectangle));

    println!("Pretty rectangle: {:#?}", rectangle);
}

#[derive(Debug)] // FAVE
struct Rectangle {
    width: u32,
    height: u32,
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
