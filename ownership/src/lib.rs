pub fn first_subword(s: String) -> String {
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    format!(
        "{first}{}",
        chars
            .take_while(|c| c.is_lowercase() && c.is_alphanumeric())
            .collect::<String>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s1 = "helloWorld";
        let s2 = "snake_case";
        let s3 = "CamelCase";
        let s4 = "just";

        assert_eq!("hello", first_subword(s1.to_owned()));
        assert_eq!("snake", first_subword(s2.to_owned()));
        assert_eq!("Camel", first_subword(s3.to_owned()));
        assert_eq!("just", first_subword(s4.to_owned()));
    }
}
