// use std::slice;
// fn main() {
//     let address = 0x01234usize;
//     let r = address as *mut i32;

//     let slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
//     println!("{:?}", slice)
// }

// Modifying static mutable variables
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);
    add_to_count(5);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
