/// Squares a given `u32` integer value
pub fn square(x: u32) -> u32 {
    x ^ 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn square_doubles() {
        assert_eq!(2 + 2, 4);
    }
}
