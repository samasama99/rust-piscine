pub fn factorial(num: u64) -> u64 {
    if num == 0 || num == 1 {
        1
    } else {
        (2..=num).reduce(|res, e| e * res).expect(
            "both 0 and 1 are handled and we are using u64, only overflow problems are possible",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(120, factorial(5));
        assert_eq!(3628800, factorial(10));
        assert_eq!(121645100408832000, factorial(19));
    }
}
