pub fn stars(n: u32) -> String {
    "*".repeat(2usize.pow(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("**", stars(1));
        assert_eq!("****************", stars(4));
        assert_eq!("********************************", stars(5));
    }
}
