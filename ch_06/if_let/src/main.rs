fn main() {
   
    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("matcH: three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("if let: three");
    }

}
