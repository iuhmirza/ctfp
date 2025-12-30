pub fn identity<T>(x: T) -> T {
    x
}


#[cfg(test)]
mod tests {
    use super::identity;

    #[test]
    fn test_i32() {
        assert_eq!(identity(1), 1);
    }

    #[test]
    fn test_ne_i32() {
        assert_ne!(identity(1), 2);
    }

}
