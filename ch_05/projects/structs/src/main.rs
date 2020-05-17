fn main() {

    // TUPLE STRUCT
    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    // let black = Color(0,0, 0);
    // let origin = Point(0, 0, 0);

    // println!("black.0: {}", black.0)

    // let mut user1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     sign_in_count: 1,
    //     active: true,
    // };

    // println!("user1: {}", user1.username);
    // println!("user1: {}", user1.email);
    // println!("user1: {}", user1.sign_in_count);
    // println!("user1: {}", user1.active);

    // let user2 = build_user(String::from("hello"), String::from("world"));

    // println!("user2: {}", user2.username);
    // println!("user2: {}", user2.email);
    // println!("user2: {}", user2.sign_in_count);
    // println!("user2: {}", user2.active);

    // let user3 = build_user_from_other(user2);

    // println!("user3: {}", user3.username);
    // println!("user3: {}", user3.email);
    // println!("user3: {}", user3.sign_in_count);
    // println!("user3: {}", user3.active);


}
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn build_user_from_other(other: User) -> User {
    User {
        ..other
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true
    }
}
