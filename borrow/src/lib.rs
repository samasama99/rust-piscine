pub fn str_len(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "hello";
        let s1 = "camelCase".to_string();
        let s2 = "olá!";
        assert_eq!(5, str_len(s));
        assert_eq!(9, str_len(&s1));
        assert_eq!(4, str_len(s2));
    }
}
