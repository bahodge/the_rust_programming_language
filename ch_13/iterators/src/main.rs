// fn main() {
//     let v1 = vec![1, 2, 3];

//     let v1_iter = v1.iter();

//     println!("Got: {:?}", v1_iter);

//     for val in v1_iter {
//         println!("Got: {}", val);
//     }
// }

// fn main() {
//     // let v1 = vec![1, 2, 3];

//     // let mut v1_iter = v1.iter();

//     // println!("{:?}", v1_iter);
//     // assert_eq!(v1_iter.next(), Some(&1));
//     // assert_eq!(v1_iter.next(), Some(&2));
//     // assert_eq!(v1_iter.next(), Some(&3));
//     // assert_eq!(v1_iter.next(), None);
//     // println!("{:?}", v1_iter);

//     let v1: Vec<i32> = vec![1, 2, 3];

//     let c: Vec<_> = v1.iter().map(|x| x + 1).collect();
//     println!("{:?}", c);
// }

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // let mut counter = Counter::new();

    // println!("{}", counter.count);
    // assert_eq!(counter.next(), Some(1));
    // assert_eq!(counter.next(), Some(2));
    // assert_eq!(counter.next(), Some(3));
    // assert_eq!(counter.next(), Some(4));
    // assert_eq!(counter.next(), Some(5));
    // assert_eq!(counter.next(), None);
    // println!("{}", counter.count);

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
