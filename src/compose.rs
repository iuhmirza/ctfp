pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where 
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| {g(f(x))}
}

#[cfg(test)]
mod test {
    use super::compose;
    use super::super::identity::identity;
    
    #[test]
    fn test_compose_with_fn() {
        fn add_one<T: std::ops::Add<Output = T> + From<u8>>(x: T) -> T {
            x + T::from(1)
        }
        
        let add_two = compose(add_one::<i32>, add_one::<i32>);
        assert_eq!(add_two(0), 2);
        assert_eq!(add_two(0), add_one(0)+add_one(0));
        add_one(3);
    }
    
    #[test]
    fn test_compose_with_closures() {
        let add_two = compose(|x: i32| x+1, |x| x+1);
        
        assert_eq!(add_two(0), 2);
        assert_eq!(add_two(0), (|x| x+1)(0) + (|x| x+1)(0));
    }
    
    #[test]
    fn test_compose_with_identity() {
        
        let id_twice = compose(identity::<i32>, identity);
        assert_eq!(id_twice(1), 1);
        assert_eq!(id_twice(1), identity(1));
        
        let add_one = |x: i32| x+1;
        assert_eq!(compose(add_one, identity)(1), compose(identity, add_one)(1));
    }
}