fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part_one_example() {
        assert_eq!(1, 2);
    }
}
