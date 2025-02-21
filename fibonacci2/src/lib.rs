pub fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 2) + fibonacci(n - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, fibonacci(2));
        assert_eq!(3, fibonacci(4));
        assert_eq!(17711, fibonacci(22));
        assert_eq!(6765, fibonacci(20));
    }
}
