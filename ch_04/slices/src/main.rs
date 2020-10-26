fn main() {

    // Better implementation
    let s = String::from("world");
    let first = first_word(&s);
    println!("first: {}", first);
    
    // THIS PREVENTS BUG
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear(); // error!
    // println!("the first word is: {}", word);


    // THIS CAUSES BUG
    // let mut words = String::from("Hello World");
    // let word = first_word(&words);
    // println!("word: {}", word);
    // words.clear();
    // println!("word: {}", word);


}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}