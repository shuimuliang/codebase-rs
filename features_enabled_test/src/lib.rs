pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://doc.rust-lang.org/reference/conditional-compilation.html
    // cargo test
    // cargo test --features network
    // cargo test --features network,filesystem
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[cfg_attr(not(feature = "network"), ignore)]
    fn test_network() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[cfg_attr(not(feature = "filesystem"), ignore)]
    fn test_filesystem() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
